#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use critical_section_lock_mut::LockMut;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use microbit::{
    hal::{
        gpio,
        pac::{self, interrupt},
        timer,
    },
    Board,
};

/// Base siren frequency in Hz.
const BASE_FREQ: u32 = 440;
/// Max rise in siren frequency in Hz.
const FREQ_RISE: u32 = 220;
/// Time for one full cycle in µs.
const RISE_TIME: u32 = 500_000;

/// These convenience types make life easier.
type SpeakerPin = gpio::Pin<gpio::Output<gpio::PushPull>>;
type SirenTimer = timer::Timer<pac::TIMER0>;

/// The current state of the siren. Updated by the interrupt
/// handler when running.
struct Siren {
    /// The timer being used by the siren.
    timer: SirenTimer,
    /// The MB2 speaker pin. Needs to be owned
    /// here for the interrupt handler.
    speaker_pin: SpeakerPin,
    /// Is the speaker pin currently high or low?
    pin_high: bool,
    /// Time in µs since the start of the current siren cycle.
    cur_time: u32,
}

impl Siren {
    /// Make a new siren with the given peripherals.
    fn new(speaker_pin: SpeakerPin, timer: SirenTimer) -> Self {
        Self {
            timer,
            speaker_pin,
            pin_high: false,
            cur_time: 0,
        }
    }

    /// Start the siren running.
    fn start(&mut self) {
        self.speaker_pin.set_low().unwrap();
        self.pin_high = false;
        self.cur_time = 0;
        self.timer.enable_interrupt();
        // The timer interval is in ticks.
        // The [nrf52833_hal] timer is hard-wired to 1M ticks/sec.
        self.timer.start(1_000_000 / BASE_FREQ);
    }

    /// Stop the siren.
    fn stop(&mut self) {
        self.timer.disable_interrupt();
    }

    /// Step the siren to the current speaker state change.
    /// This is normally called from the timer interrupt.
    fn step(&mut self) {
        // Flip the speaker pin.
        if self.pin_high {
            self.speaker_pin.set_low().unwrap();
            self.pin_high = false;
        } else {
            self.speaker_pin.set_high().unwrap();
            self.pin_high = true;
        }

        // Figure out the next period. The math is a little
        // special here.

        // First, wrap to the next siren cycle if needed.
        while self.cur_time >= 2 * RISE_TIME {
            self.cur_time -= 2 * RISE_TIME;
        }
        // Next, figure out where we are in the current siren cycle.
        let cycle_time = if self.cur_time < RISE_TIME {
            self.cur_time
        } else {
            2 * RISE_TIME - self.cur_time
        };
        // Finally, calculate the frequency and period.
        let frequency = BASE_FREQ + FREQ_RISE * cycle_time / RISE_TIME;
        let period = 1_000_000 / frequency;

        // Anticipate the time of the next interrupt.
        self.cur_time += period / 2;

        // Make sure to clear the current interrupt before
        // starting the next one, else you might get interrupted
        // again immediately.
        self.timer.reset_event();
        self.timer.start(period / 2);
    }
}

/// The siren. Accessible from both the interrupt handler
/// and the main program.
static SIREN: LockMut<Siren> = LockMut::new();

/// The timer interrupt for the siren. Just steps the siren.
#[interrupt]
fn TIMER0() {
    SIREN.with_lock(|siren| siren.step());
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    // It is convenient to use a `degrade()`ed pin
    // to avoid having to deal with the type of the
    // speaker pin, rather than looking it up:
    // the pin is stored globally in `SIREN`, so its
    // size must be known.
    //
    // This does lose type safety, but that is unlikely
    // to matter after this point.
    let speaker_pin = board
        .speaker_pin
        .into_push_pull_output(gpio::Level::Low)
        .degrade();
    let timer0 = timer::Timer::new(board.TIMER0);
    let mut timer1 = timer::Timer::new(board.TIMER1);

    // Set up the NVIC to handle interrupts.
    unsafe { pac::NVIC::unmask(pac::Interrupt::TIMER0) };
    pac::NVIC::unpend(pac::Interrupt::TIMER0);

    // Place the siren struct where the interrupt handler can find it.
    let siren = Siren::new(speaker_pin, timer0);
    SIREN.init(siren);

    // Start the siren and do the countdown.
    SIREN.with_lock(|siren| siren.start());
    for t in (1..=10).rev() {
        rprintln!("{}", t);
        timer1.delay_ms(1_000);
    }
    rprintln!("launch!");
    SIREN.with_lock(|siren| siren.stop());

    loop {
        asm::wfi();
    }
}

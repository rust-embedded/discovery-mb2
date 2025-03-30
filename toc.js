// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="index.html">Introduction</a></li><li class="chapter-item expanded "><a href="01-background/index.html"><strong aria-hidden="true">1.</strong> Background</a></li><li class="chapter-item expanded "><a href="02-requirements/index.html"><strong aria-hidden="true">2.</strong> Hardware/knowledge requirements</a></li><li class="chapter-item expanded "><a href="03-setup/index.html"><strong aria-hidden="true">3.</strong> Setting up a development environment</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="03-setup/linux.html"><strong aria-hidden="true">3.1.</strong> Linux</a></li><li class="chapter-item expanded "><a href="03-setup/windows.html"><strong aria-hidden="true">3.2.</strong> Windows</a></li><li class="chapter-item expanded "><a href="03-setup/macos.html"><strong aria-hidden="true">3.3.</strong> macOS</a></li><li class="chapter-item expanded "><a href="03-setup/verify.html"><strong aria-hidden="true">3.4.</strong> Verify the installation</a></li><li class="chapter-item expanded "><a href="03-setup/IDE.html"><strong aria-hidden="true">3.5.</strong> Setting up your IDE</a></li></ol></li><li class="chapter-item expanded "><a href="04-meet-your-hardware/index.html"><strong aria-hidden="true">4.</strong> Meet your hardware</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="04-meet-your-hardware/microbit-v2.html"><strong aria-hidden="true">4.1.</strong> micro:bit v2</a></li><li class="chapter-item expanded "><a href="04-meet-your-hardware/terminology.html"><strong aria-hidden="true">4.2.</strong> Rust Embedded terminology</a></li></ol></li><li class="chapter-item expanded "><a href="05-meet-your-software/index.html"><strong aria-hidden="true">5.</strong> Meet your software</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="05-meet-your-software/build-it.html"><strong aria-hidden="true">5.1.</strong> Build it</a></li><li class="chapter-item expanded "><a href="05-meet-your-software/flash-it.html"><strong aria-hidden="true">5.2.</strong> Flash it</a></li><li class="chapter-item expanded "><a href="05-meet-your-software/debug-it.html"><strong aria-hidden="true">5.3.</strong> Debug it</a></li><li class="chapter-item expanded "><a href="05-meet-your-software/light-it-up.html"><strong aria-hidden="true">5.4.</strong> Light it up</a></li></ol></li><li class="chapter-item expanded "><a href="06-hello-world/index.html"><strong aria-hidden="true">6.</strong> Hello World</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="06-hello-world/toggle-it.html"><strong aria-hidden="true">6.1.</strong> Toggle it</a></li><li class="chapter-item expanded "><a href="06-hello-world/spin-wait.html"><strong aria-hidden="true">6.2.</strong> Spin wait</a></li><li class="chapter-item expanded "><a href="06-hello-world/nop.html"><strong aria-hidden="true">6.3.</strong> NOP</a></li><li class="chapter-item expanded "><a href="06-hello-world/timers.html"><strong aria-hidden="true">6.4.</strong> Timers</a></li><li class="chapter-item expanded "><a href="06-hello-world/portability.html"><strong aria-hidden="true">6.5.</strong> Portability</a></li><li class="chapter-item expanded "><a href="06-hello-world/board-support-crate.html"><strong aria-hidden="true">6.6.</strong> Board support crate</a></li></ol></li><li class="chapter-item expanded "><a href="07-registers/index.html"><strong aria-hidden="true">7.</strong> Registers</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="07-registers/rtrm.html"><strong aria-hidden="true">7.1.</strong> RTRM</a></li><li class="chapter-item expanded "><a href="07-registers/misoptimization.html"><strong aria-hidden="true">7.2.</strong> (mis)Optimization</a></li><li class="chapter-item expanded "><a href="07-registers/bad-address.html"><strong aria-hidden="true">7.3.</strong> 0xBAAAAAAD address</a></li><li class="chapter-item expanded "><a href="07-registers/spooky-action-at-a-distance.html"><strong aria-hidden="true">7.4.</strong> Spooky action at a distance</a></li><li class="chapter-item expanded "><a href="07-registers/type-safe-manipulation.html"><strong aria-hidden="true">7.5.</strong> Type safe manipulation</a></li></ol></li><li class="chapter-item expanded "><a href="08-led-roulette/index.html"><strong aria-hidden="true">8.</strong> LED roulette</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="08-led-roulette/the-challenge.html"><strong aria-hidden="true">8.1.</strong> The challenge</a></li><li class="chapter-item expanded "><a href="08-led-roulette/my-solution.html"><strong aria-hidden="true">8.2.</strong> My solution</a></li></ol></li><li class="chapter-item expanded "><a href="09-serial-communication/index.html"><strong aria-hidden="true">9.</strong> Serial communication</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="09-serial-communication/nix-tooling.html"><strong aria-hidden="true">9.1.</strong> *nix tooling</a></li><li class="chapter-item expanded "><a href="09-serial-communication/windows-tooling.html"><strong aria-hidden="true">9.2.</strong> Windows tooling</a></li></ol></li><li class="chapter-item expanded "><a href="10-uart/index.html"><strong aria-hidden="true">10.</strong> UART</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="10-uart/send-a-single-byte.html"><strong aria-hidden="true">10.1.</strong> Send a single byte</a></li><li class="chapter-item expanded "><a href="10-uart/send-a-string.html"><strong aria-hidden="true">10.2.</strong> Send a string</a></li><li class="chapter-item expanded "><a href="10-uart/naive-approach-write.html"><strong aria-hidden="true">10.3.</strong> Naive approach and write!</a></li><li class="chapter-item expanded "><a href="10-uart/receive-a-single-byte.html"><strong aria-hidden="true">10.4.</strong> Receive a single byte</a></li><li class="chapter-item expanded "><a href="10-uart/echo-server.html"><strong aria-hidden="true">10.5.</strong> Echo server</a></li><li class="chapter-item expanded "><a href="10-uart/reverse-a-string.html"><strong aria-hidden="true">10.6.</strong> Reverse a string</a></li><li class="chapter-item expanded "><a href="10-uart/my-solution.html"><strong aria-hidden="true">10.7.</strong> My solution</a></li></ol></li><li class="chapter-item expanded "><a href="11-i2c/index.html"><strong aria-hidden="true">11.</strong> I2C</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="11-i2c/the-general-protocol.html"><strong aria-hidden="true">11.1.</strong> The general protocol</a></li><li class="chapter-item expanded "><a href="11-i2c/lsm303agr.html"><strong aria-hidden="true">11.2.</strong> LSM303AGR</a></li><li class="chapter-item expanded "><a href="11-i2c/read-a-single-register.html"><strong aria-hidden="true">11.3.</strong> Read a single register</a></li><li class="chapter-item expanded "><a href="11-i2c/using-a-driver.html"><strong aria-hidden="true">11.4.</strong> Using a driver</a></li><li class="chapter-item expanded "><a href="11-i2c/the-challenge.html"><strong aria-hidden="true">11.5.</strong> The challenge</a></li><li class="chapter-item expanded "><a href="11-i2c/my-solution.html"><strong aria-hidden="true">11.6.</strong> My solution</a></li></ol></li><li class="chapter-item expanded "><a href="12-led-compass/index.html"><strong aria-hidden="true">12.</strong> LED compass</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="12-led-compass/magnitude.html"><strong aria-hidden="true">12.1.</strong> Magnitude</a></li><li class="chapter-item expanded "><a href="12-led-compass/the-challenge.html"><strong aria-hidden="true">12.2.</strong> The challenge</a></li><li class="chapter-item expanded "><a href="12-led-compass/my-solution.html"><strong aria-hidden="true">12.3.</strong> My solution</a></li></ol></li><li class="chapter-item expanded "><a href="13-punch-o-meter/index.html"><strong aria-hidden="true">13.</strong> Punch-o-meter</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="13-punch-o-meter/gravity-is-up.html"><strong aria-hidden="true">13.1.</strong> Gravity is up?</a></li><li class="chapter-item expanded "><a href="13-punch-o-meter/the-challenge.html"><strong aria-hidden="true">13.2.</strong> The challenge</a></li><li class="chapter-item expanded "><a href="13-punch-o-meter/my-solution.html"><strong aria-hidden="true">13.3.</strong> My solution</a></li></ol></li><li class="chapter-item expanded "><a href="14-snake-game/index.html"><strong aria-hidden="true">14.</strong> Snake game</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="14-snake-game/game-logic.html"><strong aria-hidden="true">14.1.</strong> Game logic</a></li><li class="chapter-item expanded "><a href="14-snake-game/controls.html"><strong aria-hidden="true">14.2.</strong> Controls</a></li><li class="chapter-item expanded "><a href="14-snake-game/nonblocking-display.html"><strong aria-hidden="true">14.3.</strong> Non-blocking display</a></li><li class="chapter-item expanded "><a href="14-snake-game/final-assembly.html"><strong aria-hidden="true">14.4.</strong> Final assembly</a></li></ol></li><li class="chapter-item expanded "><a href="explore.html"><strong aria-hidden="true">15.</strong> What&#39;s left for you to explore</a></li><li class="chapter-item expanded affix "><li class="spacer"></li><li class="chapter-item expanded affix "><a href="appendix/1-general-troubleshooting/index.html">General troubleshooting</a></li><li class="chapter-item expanded affix "><a href="appendix/2-how-to-use-gdb/index.html">How to use GDB</a></li><li class="chapter-item expanded affix "><a href="appendix/3-mag-calibration/index.html">Magnetometer Calibration</a></li><li class="chapter-item expanded affix "><li class="spacer"></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);

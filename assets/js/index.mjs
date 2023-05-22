import { h, render, Component } from '/vendor/mjs/preact/preact.mjs'
import htm from '/vendor/mjs/htm/htm.mjs'

const html = htm.bind(h)

function App(props) {
    return html`
    <div class="pure-g">
        <div class="pure-u-1">
            <p class="section-title">cpu-usage</p>
            ${props.cpus.map((cpu, idx) => {
                return html`
                    <div class="cpu-usage-bar pure-u-1-4">
                        <div class="cpu-usage-bar-inner" style="width: ${cpu}%;"></div>
                        <span class="cpu-usage-text">CPU ${idx}: ${cpu.toFixed(2)}%</span>
                    </div>
                `
            })}
        </div>
        <div class="pure-u-1">
            <p class="section-title">processes</p>
        </div>
    </div>
    `
}

let urlCpuUsage = new URL('/ws/cpu', window.location.href)
urlCpuUsage.protocol = urlCpuUsage.protocol.replace('http', 'ws')

let ws = new WebSocket(urlCpuUsage.href)
ws.onmessage = (ev) => {
    let json = JSON.parse(ev.data)
    render(html`<${App} cpus=${json} />`, document.querySelector('preact-view'))
}
ws.onclose = function(e) {
    console.log('Socket is closed. Reconnect will be attempted in 1 second.', e.reason);
    setTimeout(function() {
      connect();
    }, 1000);
  };

ws.onerror = function(err) {
console.error('Socket encountered error: ', err.message, 'Closing socket');
ws.close();
};
import { h, Component, render } from 'https://esm.sh/preact'
import htm from 'https://esm.sh/htm'

const html = htm.bind(h)

function App(props) {
    return html`
    <div>
        ${props.cpus.map((cpu, idx) => {
            return html`
                <div class="cpu-usage-bar">
                    <div class="cpu-usage-bar-inner" style="width: ${cpu}%;"></div>
                    <span class="cpu-usage-text">CPU ${idx}: ${cpu.toFixed(2)}%</span>
                </div>`
        })}
    </div>
    `
}

let url = new URL('/ws/cpu', window.location.href)
url.protocol = url.protocol.replace('http', 'ws')

let ws = new WebSocket(url.href)
ws.onmessage = (ev) => {
    let json = JSON.parse(ev.data)
    render(html`<${App} cpus=${json} />`, document.querySelector('preact-view'))

}

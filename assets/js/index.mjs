import { h, Component, render } from 'https://esm.sh/preact';
import htm from 'https://esm.sh/htm';

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

setInterval(async () => {
    let response = await fetch('/api/cpu')

    if (response.status !== 200) {
        throw new Error(`HTTP error! status: ${response.status}`)
    }

    let json = await response.json()
    render(html`<${App} cpus=${json} />`, document.querySelector('preact-view'))
}, 1000)

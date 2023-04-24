import { h, Component, render } from 'https://esm.sh/preact';
import htm from 'https://esm.sh/htm';

const html = htm.bind(h)

function App(props) {
    return html`
    <div>
        ${props.cpus.map((cpu) => {
            return html`<div>${cpu.toFixed(2)}% usage</div>`
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

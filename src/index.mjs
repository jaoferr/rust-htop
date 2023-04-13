import { h, Component, render } from 'https://esm.sh/preact';


setInterval(async () => {
    let response = await fetch('/api/cpu')

    if (response.status !== 200) {
        throw new Error(`HTTP error! status: ${response.status}`)
    }

    let json = await response.json()

    const app = h("pre", null, JSON.stringify(json, null, 2))
    render(app, document.body)
}, 1000)
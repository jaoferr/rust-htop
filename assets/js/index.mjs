import { h, render, Component } from '/vendor/mjs/preact/preact.mjs'
import htm from '/vendor/mjs/htm/htm.mjs'

const html = htm.bind(h)

class CPUBars extends Component {
    state = { cpus: [] }

    componentDidMount() {
        let urlCpu = new URL('/ws/cpu', window.location.href)
        urlCpu.protocol = urlCpu.protocol.replace('http', 'ws')

        this.cpuWS = new WebSocket(urlCpu)
        this.cpuWS.onmessage = (ev) => {
            this.setState({ cpus: JSON.parse(ev.data) })
        }
        this.cpuWS.onclose = function(e) {
            console.log('Socket is closed. Reconnect will be attempted in 1 second.', e.reason)
            setTimeout(function() {
                connect()
            }, 1000)
        }

        this.cpuWS.onerror = function(err) {
            console.error('Socket encountered error: ', err.message, 'Closing socket')
            this.cpuWS.close()
        }
    }

    componentWillUnmount() {
        this.cpuWS.close()
    }

    render() {
        return html`
            <div class="pure-u-1">
                <p class="section-title">cpu-usage</p>
                ${this.state.cpus.map((cpu, idx) => {
                    return html`
                        <div class="cpu-usage-bar pure-u-1-4">
                            <div class="cpu-usage-bar-inner" style="width: ${cpu}%;"></div>
                            <span class="cpu-usage-text">CPU ${idx}: ${cpu.toFixed(2)}%</span>
                        </div>
                    `
                })}
            </div>
        `
    }
}

class ProcessesList extends Component {
    state = { processes: [] }
    queryLimit = 20

    updateQueryLimit = e => {
        this.queryLimit = e.target.value
    }

    fetchProcessList = () => {
        fetch(`/api/processes?limit=${this.queryLimit}`)
        .then(async (response) => {
            this.setState({ processes: await response.json() })
        })
        .catch((error) => {
            console.log(error)
        })
    }

    componentDidMount() {
        this.fetchProcessList()
        this.updateInterval = setInterval(() => {
            this.fetchProcessList()
        }, 10000)
    }

    componentWillUnmount() {
        clearInterval(this.updateInterval)
    }

    render() {
        return html`
            <div class="pure-u-1">
                <p class="section-title">
                    processes
                    <div class="input-process-limit">
                        <input type="text" name="processLimit" id="processLimit" value="${this.queryLimit}" maxlength="3" size="3" onChange="${this.updateQueryLimit}"/>
                    </div>
                </p>
                <table class="pure-table pure-table-horizontal process-table">
                    <thead>
                        <tr>
                            <th>#</th>
                            <th>PID</th>
                            <th>Name</th>
                            <th>Memory usage (kB)</th>
                        </tr>
                    </thead>
                    <tbody>
                        ${this.state.processes.map((process, idx) => {
                            return html`
                                <tr>
                                    <td>${idx}</td>
                                    <td>${process.pid}</td>
                                    <td>${process.process_name}</td>
                                    <td>${process.memory_usage}</td>
                                </tr>
                            `
                        })}
                    </tbody>
                </table>
            </div>
        `
    }
}

class App extends Component {
    render() {
        return html`
        <div class="pure-g">
            <${CPUBars} />
            <${ProcessesList} />
        </div>
        `
    }
}

render(html`<${App} />`, document.querySelector('preact-view'))

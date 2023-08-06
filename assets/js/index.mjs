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
                <div class="pure-grid">
                    ${this.state.cpus.map((cpu, idx) => {
                        return html`
                            <div class="pure-u-1-4">
                                <div class="cpu-usage-bar">
                                    <div class="cpu-usage-bar-inner" style="width: ${cpu}%;"></div>
                                    <span class="cpu-usage-text">CPU ${idx}: ${cpu.toFixed(2)}%</span>
                                </div>
                            </div>
                        `
                    })}
                </div>
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

    truncateText = (text, maxLength = 30) => {
        if (text.length > maxLength) {
            return text.substring(0, maxLength) + '...'
        }
        return text
    }

    /**
     *
     * @param {number} memUsage
     * @returns {string}
     */
    formatMemoryUsage = (memUsage) => {
        let stringMemUsage

        if (memUsage > 1e6) {
            stringMemUsage = (memUsage / 1e6).toFixed(2) + " GB"
        } else if (memUsage > 1e3) {
            stringMemUsage = (memUsage / 1e3).toFixed(2) + " MB"
        } else {
            stringMemUsage = memUsage + " KB"
        }

        return stringMemUsage
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
                            <th>pid</th>
                            <th>name</th>
                            <th>cpu-usage</th>
                            <th>memory-usage</th>
                            <th>cmd</th>
                        </tr>
                    </thead>
                    <tbody>
                        ${this.state.processes.map((process, idx) => {
                            return html`
                                <tr>
                                    <td>${idx}</td>
                                    <td>${process.pid}</td>
                                    <td>${process.process_name}</td>
                                    <td>${process.cpu_usage.toFixed(2)}</td>
                                    <td>${this.formatMemoryUsage(process.memory_usage)}</td>
                                    <td>${this.truncateText(process.command)}</td>
                                </tr>
                            `
                        })}
                    </tbody>
                </table>
            </div>
        `
    }
}

class SystemInfo extends Component {
    state = { systemInfo: {} }

    formatUptime = (uptime) => {
        let date = new Date(uptime * 1000)
        let hours = date.getHours()
        let minutes = date.getMinutes()
        let seconds = date.getSeconds()
        return `${(hours<10) ? '0' + hours : hours}:${(minutes<10) ? '0' + minutes : minutes}:${(seconds<10) ? '0' + seconds : seconds}`
    }

    fetchSystemInfo = () => {
        fetch('/api/sysinfo')
        .then(async (response) => {
            // console.log(await response.json())
            this.setState({ systemInfo: await response.json() })
        })
        .catch((error) => {
            console.log(error)
        })
    }

    componentDidMount() {
        this.fetchSystemInfo()
        this.updateInterval = setInterval(() => {
            this.fetchSystemInfo()
        }, 10000)
    }

    componentWillUnmount() {
        clearInterval(this.updateInterval)
    }

    render() {
        return html`
            <div class="pure-u-1">
                <div class="pure-grid">
                    <div class="pure-u-1-4">
                        <div class="system-info-text">
                            <b>System:</b> ${this.state.systemInfo.os_name}
                        </div>
                    </div>
                    <div class="pure-u-1-4">
                        <div class="system-info-text">
                            <b>Total memory:</b> ${this.state.systemInfo.total_memory} MB
                        </div>
                    </div>
                    <div class="pure-u-1-4">
                        <div class="system-info-text">
                            <b>Available memory:</b> ${this.state.systemInfo.available_memory} MB
                        </div>
                    </div>
                    <div class="pure-u-1-4">
                        <div class="system-info-text">
                            <b>Uptime:</b> ${this.formatUptime(this.state.systemInfo.uptime)}
                        </div>
                    </div>
                </div>
            </div>
        `
    }
}

class DiskInfo extends Component {
    state = { disks: [] }

    formatDiskSpace = (diskSpace) => {
        if (diskSpace > 10000) {
            return (diskSpace / 1000).toFixed(2) + ' GB'
        }
        return diskSpace + ' MB'
    }

    fetchDiskInfo = () => {
        fetch('/api/diskinfo')
        .then(async (response) => {
            this.setState({ disks: await response.json() })
        })
        .catch((error) => {
            console.log(error)
        })
    }

    componentDidMount() {
        this.fetchDiskInfo()
        this.updateInterval = setInterval(() => {
            this.fetchDiskInfo()
        }, 30000)
    }

    componentWillUnmount() {
        clearInterval(this.updateInterval)
    }

    render() {
        return html`
            <div class="pure-u-1">
                <p class="section-title">disk-info</p>
                <div class="pure-grid">
                    ${this.state.disks.map((disk, idx) => {
                        return html`
                            <div class="pure-u-1-4">
                                <div class="disk-name-text">${disk.name}</div>
                                <div class="disk-usage-bar">
                                    <div class="disk-usage-bar-inner" style="width: ${(disk.used_space/disk.total_space).toFixed(2)*100}%;"></div>
                                    <span class="disk-usage-text">${this.formatDiskSpace(disk.used_space)} / ${this.formatDiskSpace(disk.total_space)}</span>
                                </div>
                            </div>
                        `
                    })}
                </div>
            </div>
        `
    }
}


class App extends Component {
    render() {
        return html`
        <div class="pure-g">
            <${SystemInfo} />
            <${DiskInfo} />
            <${CPUBars} />
            <${ProcessesList} />
        </div>
        `
    }
}

render(html`<${App} />`, document.querySelector('preact-view'))

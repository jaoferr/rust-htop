document.addEventListener("DOMContentLoaded", () => {
    let c = 0

    setInterval(() => {
        c += 1
        document.body.textContent = `count is at ${c}`
    }, 1000);
})
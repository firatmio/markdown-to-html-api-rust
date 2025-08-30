const textarea = document.getElementById("markdown");
const preview = document.getElementById("preview");

let timeoutId;

textarea.addEventListener("input", () => {
    clearTimeout(timeoutId);
    timeoutId = setTimeout(async () => {
        const markdown = textarea.value;
        try {
            const res = await fetch("http://127.0.0.1:8080/render", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ markdown })
            });
            if (!res.ok) throw new Error("API error: " + res.status);
            const html = await res.text();
            preview.innerHTML = html;
        } catch (err) {
            preview.innerText = "Error: " + err.message;
        }
    }, 200);
});
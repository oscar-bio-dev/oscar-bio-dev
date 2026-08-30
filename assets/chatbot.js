document.addEventListener("DOMContentLoaded", () => {
    const input = document.getElementById("chat-input");
    const history = document.getElementById("chat-history");
    
    // In a production app, the API Key wouldn't be hardcoded on the client-side like this
    // but rather injected by the backend or managed through an authenticated session.
    // For this Agent Terminal UI demo, we use the static key.
    const API_KEY = "default_secure_key_123";

    async function sendQuery(query) {
        // Add user line
        const userLine = document.createElement("div");
        userLine.className = "chat-line user-line";
        userLine.innerHTML = `<span class="prompt">oscar@bio-dev$</span> ${escapeHtml(query)}`;
        history.appendChild(userLine);
        
        input.value = "";
        input.disabled = true;

        // Add loading line
        const aiLine = document.createElement("div");
        aiLine.className = "chat-line ai-line";
        aiLine.innerHTML = `<span class="prompt">gemini-twin$</span> <span class="ai-text">Analyzing telemetry...</span><span class="typing-cursor"></span>`;
        history.appendChild(aiLine);
        history.scrollTop = history.scrollHeight;

        try {
            const res = await fetch("/api/chat", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                    "Authorization": `Bearer ${API_KEY}`
                },
                body: JSON.stringify({ message: query })
            });

            if (!res.ok) {
                throw new Error(`Connection Error: ${res.status}`);
            }

            const data = await res.json();
            typeWriterEffect(aiLine, data.reply);
        } catch (error) {
            aiLine.querySelector(".ai-text").innerText = `[ERR] ${error.message}`;
            const cursor = aiLine.querySelector(".typing-cursor");
            if (cursor) cursor.remove();
            input.disabled = false;
            input.focus();
        }
    }

    function typeWriterEffect(element, text) {
        const textSpan = element.querySelector(".ai-text");
        textSpan.innerText = "";
        let i = 0;
        
        function type() {
            if (i < text.length) {
                textSpan.innerText += text.charAt(i);
                i++;
                history.scrollTop = history.scrollHeight;
                setTimeout(type, 15); // Fast typing speed
            } else {
                const cursor = element.querySelector(".typing-cursor");
                if (cursor) cursor.remove();
                input.disabled = false;
                input.focus();
            }
        }
        type();
    }

    function escapeHtml(unsafe) {
        return unsafe
             .replace(/&/g, "&amp;")
             .replace(/</g, "&lt;")
             .replace(/>/g, "&gt;")
             .replace(/"/g, "&quot;")
             .replace(/'/g, "&#039;");
    }

    input.addEventListener("keypress", (e) => {
        if (e.key === "Enter" && input.value.trim() !== "") {
            sendQuery(input.value.trim());
        }
    });
});

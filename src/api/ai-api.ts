const API_KEY = "";

export async function callOpenAI(userText: string) {
    const prompt = "Strukturiere den folgenden Text klar. Verwende nur HTML Syntax für die Header, Listen und Text Formatierung und keine anderen Formate. Füge passende Ergänzungen hinzu, KEINE Einleitung"
    const response = await fetch("https://api.openai.com/v1/chat/completions", {

        method: "POST",
        headers: {
            "Content-Type": "application/json",
            Authorization: `Bearer ${API_KEY}`,
        },
        body: JSON.stringify({
            model: "gpt-4o-mini",
            messages: [{ role: "user", content: `${prompt}\n\n${userText}` }],
            temperature: 0.3,
        }),
    });

    const data = await response.json();
    return data.choices?.[0]?.message?.content;
}
export async function AICharts(userText: string) {
    const prompt = "Du bist ein Assistent, der Nutzereingaben analysiert und daraus automatisch das passende"
        + "Mermaid.js-Diagramm erzeugt. Wähle den am besten geeigneten Diagrammtyp basierend auf dem Input und gib ausschließlich den Mermaid.js-Code zurück."
        + "Regeln:Wenn es sich um Anteile oder Prozentwerte handelt → pie, Wenn es sich um zeitliche Abläufe handelt → gantt"
        + "Wenn es um Prozesse oder Beziehungen geht → flowchart, Wenn es sich um Kategorien und Werte handelt → bar oder line"
        + "Gib nur den Mermaid-Code-Block zurück (kein Text davor oder danach)"

    const response = await fetch("https://api.openai.com/v1/chat/completions", {

        method: "POST",
        headers: {
            "Content-Type": "application/json",
            Authorization: `Bearer ${API_KEY}`,
        },
        body: JSON.stringify({
            model: "gpt-4o-mini",
            messages: [{ role: "user", content: `${prompt}\n\n${userText}` }],
            temperature: 0.3,
        }),
    });

    const data = await response.json();
    return data.choices?.[0]?.message?.content;
}
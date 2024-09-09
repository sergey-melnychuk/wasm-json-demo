import init, { process_json, process_json_blocking, JsonProcessor, set_hook } from '../pkg/wasm_json_demo.js';

async function run() {
    await init();
    set_hook();
    const div = document.getElementById('log');

    const inputJson = JSON.stringify({ key: "url", url: "http://127.0.0.1:3000/example" });
    const outputJson = await process_json(inputJson);
    console.log("Output JSON:", outputJson);

    const output = process_json_blocking(inputJson);
    console.log("Output JSON:", output);

    const invalidJson = '{ key: "value" }';
    try {
        const outputJson = await process_json(invalidJson);
        console.log("Output JSON:", outputJson);
        dump(div, "Outpot JSON: " + outputJson);
    } catch (err) {
        console.error("Error processing JSON:", err);
        dump(div, "Error processing JSON: " + err, 'error');
    }

    let processor;
    try {
        processor = new JsonProcessor(inputJson);
        console.log("Initial data:", processor.get_data());
        dump(div, "Initial data: " + processor.get_data());
    } catch (err) {
        console.error("Error creating JsonProcessor:", err);
        dump(div, "Error creating JsonProcessor: " + err, 'error');
        return;
    }
    try {
        const processedJson = processor.process();
        console.log("Processed data:", processedJson);
        dump(div, "Processed data: " + processedJson);
    } catch (err) {
        console.error("Error processing JSON:", err);
        dump(div, "Error processing JSON: " + err, 'error');
    }
}

function dump(div, text, style) {
    let p = document.createElement('p');
    if (style != undefined) {
        p.className = style;
    }
    p.innerText = text;
    div.appendChild(p);
}

run();

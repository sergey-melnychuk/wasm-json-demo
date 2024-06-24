import init, { process_json, JsonProcessor } from './node_modules/wasm-json-demo/wasm_json_demo.js';

async function run() {
    await init();
    const div = document.getElementById('log');

    const inputJson = JSON.stringify({ key: "value" });
    const outputJson = process_json(inputJson);
    console.log("Output JSON:", outputJson);

    const invalidJson = '{ key: "value" }';
    try {
        const outputJson = process_json(invalidJson);
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

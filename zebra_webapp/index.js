import * as wasm from "zebra_wasm";

export function verifySignature(messageString) {
    return wasm.verify_signature(messageString);
}

export function parseLineForIdentifier(input) {
    try {
        const result = wasm.parse_line_for_identifier(input);
        return [result.prefix, result.identifier, result.suffix];
    } catch (error) {
        console.error("Error parsing line:", error);
        return [input, '', ''];
    }
}

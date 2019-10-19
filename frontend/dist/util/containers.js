"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
// TODO: store this as an env
const HOST = "http://localhost:3000";
function getPath(warehouse = null, pallet = null, box = null, unit = null) {
    let url = "";
    url += warehouse ? `/warehouse/${warehouse}` : "";
    url += pallet ? `/pallet/${pallet}` : "";
    url += box ? `/box/${box}` : "";
    url += unit ? `/unit/${unit}` : "";
    const text = url.replace(/\//g, " > ");
    url = `${HOST}${url}`;
    return {
        url,
        text
    };
}
exports.getPath = getPath;
//# sourceMappingURL=containers.js.map
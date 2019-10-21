"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const seed_1 = require("../util/seed");
/**
 * GET /
 * Pallets page.
 */
exports.index = (req, res) => {
    // generate sample crate data
    const pallets = seed_1.generateItems(5, 20);
    const warehouseDesc = "Warehouse #2 Primarily focuses on storing complete products...";
    // send to user
    console.log(pallets);
    res.render("items", {
        title: "Pallet Search",
        items: pallets,
        parentDesc: warehouseDesc,
        itemType: "box"
    });
};
//# sourceMappingURL=pallets.js.map
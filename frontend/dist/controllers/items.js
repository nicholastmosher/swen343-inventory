"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const seed_1 = require("../util/seed");
/**
 * GET /
 * Warehouses page.
 */
const warehouses = (req, res) => {
    // send to user
    res.render("items", {
        title: "Warehouse Search",
        items: seed_1.generateItems(5, 20),
        itemType: "warehouse",
        containerPath: []
    });
};
/**
 * GET /warehouse/:id
 * Pallets page.
 */
const pallets = (req, res) => {
    // get current id path
    const id = parseInt(req.params.id);
    // send to user
    res.render("items", {
        title: "Pallet Search",
        items: seed_1.generateItems(5, 20),
        parentDesc: `Warehouse #${id} primarily focuses on storing complete products...`,
        itemType: "pallet",
        containerPath: [id]
    });
};
/**
 * GET /warehouse/:warehouseId/pallet/:id
 * Boxes page.
 */
const boxes = (req, res) => {
    // get current id path
    const id = parseInt(req.params.id);
    const warehouseId = parseInt(req.params.warehouseId);
    // send to user
    res.render("items", {
        title: "Box Search",
        items: seed_1.generateItems(6, 10),
        parentDesc: `Pallet #${id} Primarily focuses on storing complete products...`,
        itemType: "box",
        containerPath: [warehouseId, id]
    });
};
/**
 * GET /warehouse/:warehouseId/pallet/:palletId/boxes/:id
 * Units page.
 */
const units = (req, res) => {
    // get current id path
    const id = parseInt(req.params.id);
    const palletId = parseInt(req.params.palletId);
    const warehouseId = parseInt(req.params.warehouseId);
    const items = seed_1.generateItems(5, 20);
    // add 'type' flag for part or product to the item
    const isPart = Math.floor(Math.random() * 2);
    const type = isPart ? "part" : "product";
    items.forEach((item) => { item.type = type; });
    // send to user
    res.render("items", {
        title: "Unit Search",
        items,
        parentDesc: `Box #${id} primarily focuses on storing complete products...`,
        itemType: "unit",
        containerPath: [warehouseId, palletId, id]
    });
};
// export routes
exports.default = {
    warehouses,
    pallets,
    boxes,
    units
};
//# sourceMappingURL=items.js.map
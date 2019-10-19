"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const seed_1 = require("../util/seed");
/**
 * GET /warehouses/:warehouseId/pallet/:palletId/boxes/:id
 * Unit page.
 */
function getDetails(req, res) {
    // get current id path
    const id = parseInt(req.params.id);
    const boxId = parseInt(req.params.boxId);
    const palletId = parseInt(req.params.palletId);
    const warehouseId = parseInt(req.params.warehouseId);
    // send to user
    res.render("unit", {
        title: `Part #${id}`,
        item: seed_1.generateItems(1, id)[0],
        cost: 1000,
        compName: "Part D",
        components: ["Part A", "Part B"],
        description: "This part is used ",
        box: boxId,
        pallet: palletId,
        warehouse: warehouseId,
        containerPath: [warehouseId, palletId, boxId, id]
    });
}
;
exports.default = {
    getDetails
};
//# sourceMappingURL=unit.js.map
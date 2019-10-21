"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
function randomDate() {
    const start = new Date(2019, 0, 9);
    const end = new Date();
    return new Date(start.getTime() + Math.random() * (end.getTime() - start.getTime()));
}
function addDays(date, days) {
    const result = new Date(date);
    result.setDate(result.getDate() + days);
    return result;
}
function generateItems(count, idOffset = 0) {
    const TYPES = ["F", "C", "H", "A", "B"]; // TODO: fix these types
    const items = [];
    for (let i = 0; i < count; i++) {
        // setup pallet data
        const date = randomDate(); // start and end date
        const types = []; // types assigned to pallet
        for (let k = 0; k < TYPES.length; k++)
            if (Math.floor(Math.random() * 2)) // 50% chance
                types.push(TYPES[k]);
        // create and add box to pallet
        items.push({
            id: i + idOffset,
            cost: Math.floor(Math.random() * 20) * 50,
            dateOrdered: date.toLocaleDateString(),
            dateArrived: addDays(date, Math.floor(Math.random() * 7)).toLocaleDateString(),
            description: `Reordered automatically (${Math.floor(Math.random() * 20)}/50)`,
            numBoxes: 6,
            types
        });
    }
    ;
    return items;
}
exports.generateItems = generateItems;
//# sourceMappingURL=seed.js.map
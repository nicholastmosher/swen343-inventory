"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
/**
 * GET /catalog
 * Catalog page.
 */
const catalog = (req, res) => {
    // hit items, get all of them, put them in this catalog array
    const catalog = [
        {
            id: "Product A",
            cost: "1000",
            description: "Product A will make all of your dreams come true with its drastically wonderful components.",
            tags: ["F", "C", "A"],
            parts: ["Part 1", "Part 2"],
        },
        {
            id: "Product B",
            cost: "500",
            description: "Not everyone can afford Product A, so theres Product B. Now they can and shall afford at least something.",
            tags: ["F", "H"],
            parts: ["Part 2"]
        },
        {
            id: "Product C",
            cost: "1250",
            description: "This product is exclusive for members who want to be extra... flashy. All of the greats into one.",
            tags: ["F", "H", "B"],
            parts: ["Part 1", "Part 2", "Part 3"]
        }
    ];
    // send to user
    res.render("catalog", {
        title: "Item Catalog",
        items: catalog,
    });
};
/**
 * GET /catalog/create
 * Create a new item
 */
const createItem = (req, res) => {
    // send to user
    res.render("create_item", {
        title: "Create Item"
    });
};
/**
 * GET /create/warehouse
 * TODO probs should move this
 */
const createWarehouse = (req, res) => {
    // send to user
    res.render("add_warehouse", {
        title: "Add Warehouse"
    });
};
// export routes
exports.default = {
    catalog,
    createItem,
    createWarehouse,
};
//# sourceMappingURL=catalog.js.map
import * as mc from '@minecraft/server';

/** @type {mc.ItemCustomComponent} */
const amethystSwordComponent = {
    onUse: (data) => {
        const player = data.source;
        const item = data.itemStack;

        const block = player.getBlockFromViewDirection()?.block;
        /** @type {mc.EntityInventoryComponent} */
        const inv = player.getComponent(mc.EntityComponentTypes.Inventory);

        if (block === undefined) {
            if (item.getDynamicProperty("vio:charge") > 0 && item.getDynamicProperty("vio:charge") !== undefined) {
                item.setDynamicProperty("vio:charge", (item.getDynamicProperty("vio:charge") ?? 1) - 1);
                player.applyKnockback(0, 0, 0, 2.5);
                inv.container.setItem(player.selectedSlotIndex, item);
            }
            return;
        }

        if (block.typeId === "minecraft:amethyst_block" && block.above().typeId === "minecraft:chain" && block.above(2).typeId === "minecraft:heavy_core") {
            item.setDynamicProperty("vio:charge", (item.getDynamicProperty("vio:charge") ?? 0) + 1);

            inv.container.setItem(player.selectedSlotIndex, item);
        } else {if (item.getDynamicProperty("vio:charge") > 0 && item.getDynamicProperty("vio:charge") !== undefined) {
            item.setDynamicProperty("vio:charge", (item.getDynamicProperty("vio:charge") ?? 1) - 1);
            player.applyKnockback(0, 0, 0, 2.5);
            inv.container.setItem(player.selectedSlotIndex, item);
        }}
    }
};

mc.world.beforeEvents.worldInitialize.subscribe((data) => {
    data.itemComponentRegistry.registerCustomComponent("vio:amethyst_sword", amethystSwordComponent);
})

// mc.world.afterEvents.itemUse.subscribe(() => {
//     mc.world.sendMessage("This is working! Violet is pairing the scripts!")
// })
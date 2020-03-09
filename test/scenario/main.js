const { mainConfig } = require("../config");

module.exports = scenario => {
  scenario("root linking pattern", async (s, t) => {
    const { alice } = await s.players({
      alice: mainConfig
    });

    await alice.spawn();

    // create base entry
    const base_result = await alice.call("sandbox", "main", "create_my_entry", {
      entry: {
        content: "foo"
      }
    });
    t.ok(base_result.Ok);

    // create target entry
    const target_result = await alice.call(
      "sandbox",
      "main",
      "create_my_entry",
      {
        entry: {
          content: "bar"
        }
      }
    );
    t.ok(target_result.Ok);

    // link entries (will link entries at their root)
    const link_entries_at_root = await alice.call(
      "sandbox",
      "main",
      "link_my_entries",
      {
        base_address: base_result.Ok,
        target_address: target_result.Ok
      }
    );
    t.ok(link_entries_at_root.Ok);

    await s.consistency();

    // check that base has linked to the target
    const get_linked_result = await alice.call(
      "sandbox",
      "main",
      "get_linked_my_entries",
      {
        base_address: base_result.Ok
      }
    );
    t.ok(get_linked_result.Ok);
    t.deepEqual(get_linked_result.Ok.links.length, 1);

    // update the base
    const update_base = await alice.call("sandbox", "main", "update_my_entry", {
      entry: {
        content: "fizz"
      },
      address: base_result.Ok
    });
    t.ok(update_base.Ok);

    await s.consistency();

    // check that we can still retrieve target addresses with this updated base address
    const get_linked_result_2 = await alice.call(
      "sandbox",
      "main",
      "get_linked_my_entries",
      {
        base_address: update_base.Ok
      }
    );
    t.ok(get_linked_result_2.Ok);
    t.deepEqual(get_linked_result_2.Ok.links.length, 1);
  });
};

/// NB: The try-o-rama config patterns are still not quite stabilized.
/// See the try-o-rama README [https://github.com/holochain/try-o-rama]
/// for a potentially more accurate example

const path = require("path");
const tape = require("tape");

const {
  Orchestrator,
  tapeExecutor,
  singleConductor,
  combine
} = require("@holochain/tryorama");

const orchestrator = new Orchestrator();

require("./scenario/main")(orchestrator.registerScenario);

orchestrator.run();

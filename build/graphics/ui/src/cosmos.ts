/* Need to figure out how we'll be creating components and UIs. We want to have the ability to create multiple UIs and drive those UIs and components, on multiple devices, and have them all asynchronously synced. I think this state management should be handleb by cosmos core since it needs the ability to create UIs that span different programming implementations */

/* How shall we describe UIs? Do they need an intrinsic heirarchy? How do we deal with layering? In HTML right now, elements inside other elements have to respect their parents z-index. When we think about that that makes fairly good sense on a physical and a digital level. However there are many instances where we may like certain elements to be able to surpass their parent's layering, in order to have other elements exist inbetween them without having to be a member of that parent component. */
// let ui = [
//   {
//     type: 'button',
//     width: '100px',
//     height: '400px',
//     children: []
//   }
// ]

// // Protocol for component creation
// const protocol = {
//   id: 'string', // GENERATED Will use cosmos to generate ID
//   hash: 'string', // GENERATED Will use cosmos to generate the hash
//   groups: {},  //  Other elements that this is related to
//   type: 'string',
//   spacing: {
//       inner: [], // [x, y, left, right, top, bottom]
//       outer: [] // [x, y, left, right, top, bottom]
//   },
//   positioning: {
//       reference: null,
//       x: 0,
//       y: 0
//   }
// }

/** The create function creates a single or series of components using the data that is fed to the function. Once I figure out how to use the vuex state management function, I'll be able to have that fuel component creation */
// function create(component) {

//   if (component) {
//     // Compare component description to protocol
//     // Use what matches the protocol
//     // Use defaults for the rest
//   } else {
//     return;
//   }

//   // Generate component
//     // Try to build the component as described
//     // Register component as successfully created

    
// }

// function update() {}
// function get() {}
// function remove() {}


// create();
// update();
// get();
// remove();


// create(ui);

// loggin help
const logEnabled = true;
// list of log levels MUST HAVE UNIQUE NAMES.
// log levels are ranked based on their position in the array, with levels having later placement having higher rank (based on position in array starting at 0)
const logLevels = [
  'normal',
  'verbose',
  'extraverbose',
];
// set the current log level
const logLevel = 'verbose';
// get current logLevel ranking
const logLevelRank = logLevels.indexOf(logLevel);

function log(level: string, message: any) {
  // check if logging is enabled
  if (logEnabled) {
    // get current level ranking
    const levelRank = logLevels.indexOf(level);
    // check to make sure we've found the level in our log level array, and that the logLevel also has been found in the array
    if (levelRank >= 0 && logLevelRank >= 0) {
      // output message if level has a matching rank or lower
      if (levelRank <= logLevelRank) {
        console.log(message);
      }
    }
  }
}

export {log, logLevel, logEnabled};
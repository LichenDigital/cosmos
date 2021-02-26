import Vue from 'vue'
import App from './App.vue'

Vue.config.productionTip = false

new Vue({
  render: h => h(App),
}).$mount('#app')


/* Need to figure out how we'll be creating components and UIs. We want to have the ability to create multiple UIs and drive those UIs and components, on multiple devices, and have them all asynchronously synced. I think this state management should be handleb by cosmos core since it needs the ability to create UIs that span different programming implementations */

/* How shall we describe UIs? Do they need an intrinsic heirarchy? How do we deal with layering? In HTML right now, elements inside other elements have to respect their parents z-index. When we think about that that makes fairly good sense on a physical and a digital level. However there are many instances where we may like certain elements to be able to surpass their parent's layering, in order to have other elements exist inbetween them without having to be a member of that parent component. */
let ui = [
  {
    type: 'button',
    width: '100px',
    height: '400px',
    children: []
  }
]

// Protocol for component creation
const protocol = {
  id: 'string', // GENERATED Will use cosmos to generate ID
  hash: 'string', // GENERATED Will use cosmos to generate the hash
  groups: {},  //  Other elements that this is related to
  type: 'string',
  spacing: {
      inner: [], // [x, y, left, right, top, bottom]
      outer: [] // [x, y, left, right, top, bottom]
  },
  positioning: {
      reference: null,
      x: 0,
      y: 0
  }
}

/** The create function creates a single or series of components using the data that is fed to the function. Once I figure out how to use the vuex state management function, I'll be able to have that fuel component creation */
function create(component) {

  if (component) {
    // Compare component description to protocol
    // Use what matches the protocol
    // Use defaults for the rest
  } else {
    return;
  }

  // Generate component
    // Try to build the component as described
    // Register component as successfully created

    
}

function update() {}
function get() {}
function remove() {}


create();
update();
get();
remove();


create(ui);
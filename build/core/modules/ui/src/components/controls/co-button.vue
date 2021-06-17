<template lang="pug">
a.button(v-if='link' :href='link.destination' :target='newWindow')
  slot
button.button(v-else)
  slot
</template>

<script lang="ts">

import { defineComponent } from 'vue';

interface LinkInterface {
  destination: string;
  newWindow: boolean;
}

export default defineComponent({
  name: 'coButton',
  props: {
    link: { type: Object as () => LinkInterface }
  },
  data: function() {
    return {
      id: null,
      group: null,
      nodes: [
        {
          type: 'text',
          content: 'hello'
        },
        {
          type: 'icon',
          content: 'check-mark'
        }
      ],
      style: {
        border: {
          width: '0px',
          dash: [10, 50, 100, 30],
          color: '',
        },
        width: null,
        height: null,
        shape: {
          rectangle: null,
          square: null,
          triangle: null,
          circle: null,
          ngon: null,
          path: null
        },
      },
      actions: [
        {
          reference: 'hash |or| id |or| group |or| id &and& group |or| group &and& group etc.',
          reactions: [
            { reference: 'hash |or| id |or| group |or| id &and& group |or| group &and& group etc.' }
          ]
        }
      ]
    }
  },
  computed: {
    newWindow(): string | null  {
      if (this.link?.newWindow && this.link?.destination) {
        return '_blank';
      } else {
        return null;
      }
    }
  }
  // Need to itentify all common properties we'll need to make available to all components, and then we can probably pull that array in and load it into the props, and merge it with any component specific props

  // General properties

  // Link
  // Actions
  // States
  
  // Color
  // Sizing
  // Spacing
  // Alignment

  // Typography
    // Color
    // Sizing
    // Spacing
      // Inner
    // Weight

  // Labels - list
  // Icons - list

  // COMMON COSMOS ACTIONS AND DATA
  //     id - Grants you the ability to know where something came from and you can interact with and control a specific button
  //     hash - Grants you the ability to target buttons that are all the same
  //     groups - You've discovered the power to group items together
  // props: ['sizing', 'spacing', 'alignment', 'position', 'links', 'labels', 'icons', 'state', 'groups', 'id', 'hash']
})
</script>


<style lang="scss" scoped>


/*
||||| BUTTONS |||||

SHAPES:
  rectangular (default)
  square
  circular
  vector (experimental shapes)

VARIENTS:
  solid (default)
  outline (outline style - solid, dashed, dotted, double)
  bordered (solid w/ border)
  borderless (transparent w/o border, like outline but without border)
  underlined

MODIFIERS:
  rounded
  rounded corners
  
ANIMATIONS:
  hover
  click
  progress
  specific - to icons used
  morph (circle to rectangle, rectangle to another, border-radius transitions

STATES:
  hover
  active
  disabled

PROGRESS:
  location_top
  location_bottom
  location_left
  location_right
  style_background
  style_border (experimental)
  style_shift
  style_outside
ICON:
  icon
  icon_left
  icon_right
  icon_top
  icon_bottom
  icon_position (order in placement (flexbox  makes this easy))

DEVELOPMENT NOTES:
  Need to develop a method for handling colors pretty quickly. Unless I don't use color initially, and work on getting the foundational elements set up. (That makes pretty good sense).

  Will need to make a choice on rounded corners and ends and how those are going to be implimented accross components. There could be a couple ways of going about this:
    - Use global variables to turn them on and off system wide (override - could use a conditional in the default style)
    - Use specific class names (right now) to enable and disable them
    - Both
  I think it comes down to whether this is an option that would want to be used as a class or only as a variable in SASS. Use of the class could avoid touching the SASS, use of the variable would apply the change site-wide on eligible elements. They could be both used, with the use of classes, perhaps not being as strong, as it requires using that class on every button you create, and they will probably be all the same site-wide..

  This also requires thought put into how classes will be generally accross components to style elements. If I use a .co-rounded-corners on a button, it should have the same result as if I use it on a check box, an input, a container etc.

  I will want to be handling padding on buttons and inputs per styles. For instance a round button needs to have equal padding on both top and bottom, aka it needs to have its left and right padding be the same as the top (to ensure that the overall button height stays the same as it would normally)

*/

.button {

  cursor: pointer;
  user-select: none;

  // Temp use of color
  background: white;
  color: white; // This will need to be calculated based on the lightness of the primary color chosen

  position: relative;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  vertical-align: top;
  padding: .5rem 1rem;

  font-family: sans-serif;
  font-size: 1.4rem;
  color: inherit;
  line-height: 1.6;
  margin-right: 1rem;
  
  border: solid 2px black;
  outline: none;

  border-radius: 0;
}

</style>

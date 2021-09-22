<template lang="pug">
//- label(for='standard-dropdown') Dropdown
.dropdown
  select(ref='dropdown' @click='optionsToggle()' v-model='selected')
    option(v-for='(option, index) in options' :key='index' :value='option') {{ option }}
  .options(v-show='optionsDisplay' :style='{visibility: optionsVisible ? "visible" : "hidden"}' ref='options')
    ul
      li(v-for='(option, index) in options' :key='index' @click='optionSelected(option)') {{ option }}
</template>

<script lang="ts">

import { defineComponent } from 'vue';

export default defineComponent({
  name: 'dropdown',
  data() {
    return {
      optionsOpenDirection: 'down',
      optionsDisplay: false,
      optionsVisible: false,
      selected: '',
      options: [
        'option 1',
        'option 2',
        'option 3',
        'option 4',
        'option 5',
        'option 6',
      ]
    };
  },
  methods: {
    optionSelected(option: string) {
      this.optionsToggle();
      this.selected = option;
    },
    optionsPlacement() {
      // Get location of the dropdown input
      let location = this.$refs.dropdown.getBoundingClientRect();
      // Calculate the distance to the window
      let distance = {
        top: location.top,
        bottom: window.innerHeight - location.bottom,
        left: location.left,
        right: window.innerWidth - location.right,
      }
      console.log(distance);
      // Set the display to block (while the visiblility is still false) so we can calculate the size of the dropdown
      this.optionsDisplay = true;
      // Get the size of the options menu
      let optionsSize = this.$refs.options.getBoundingClientRect();
      console.log(optionsSize); 
    },
    optionsToggle() {
      // Check if options are visible. If not, then calculate placement before showing
      if(this.optionsVisible === false) {
        this.optionsPlacement();
      }
      // Show options
      this.optionsVisible = !this.optionsVisible;
    }
  }
});

</script>

<style lang="scss">
.dropdown {
  display: grid;
  grid-template-areas: 'select';
  align-items: center;
  position: relative;
  
  &::before {
    grid-area: select;
    justify-self: end;
    content: '';
    width: 1.2rem;
    height: .75rem;
    margin-right: 1rem;
    background-color: black;    
    clip-path: polygon(100% 0%, 0% 0%, 50% 100%);
    pointer-events: none;
  }

  option {
    display: none;
  }
}
select {
  grid-area: select;
  appearance: none;
  background: white;
  border: 2px solid black;
  padding: .5rem 1rem;
  width: 100%;
  font-family: 'Open Sans', sans-serif;
  font-size: 1.4rem;
  color: inherit;
  line-height: 1.6;
  outline: none;

  &:focus,&:active {
    box-shadow: 0 0 0 2px #888;
  }
}
.options {
  position: absolute; // Perhaps allow for this to be relatively positioned for people who want it inserted instead of on top of elements
  left: 0; top: calc(100% - 2px);
  width: 100%;
  background-color: white;
  ul {
    border: 2px solid black;
  }
  li {
    display: block;
    padding: .5rem 1rem;
    line-height: 1.6;
    &:nth-child(2n) {
      background: #eee;
    }
  }
}
</style>
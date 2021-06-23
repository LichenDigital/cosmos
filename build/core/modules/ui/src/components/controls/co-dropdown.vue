<template lang="pug">
//- label(for='standard-dropdown') Dropdown
.dropdown
  select(id='standard-dropdown')
    option(value='Option 1') Option 1
    option(value='Option 2') Option 2
    option(value='Option 3') Option 3
    option(value='Option 4') Option 4
    option(value='Long Option') Option that has too long of a value to fit
</template>

<script lang="ts">

import { defineComponent } from 'vue';

export default defineComponent({
  name: 'dropdown',
});

</script>

<style lang="scss">
.dropdown {
  display: grid;
  grid-template-areas: 'select';
  align-items: center;
  
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
}
select {
  grid-area: select;
  appearance: none;
  background: white;
  border: 2px solid black;
  padding: .5rem 1rem;
  width: 100%;
  font-family: sans-serif;
  font-size: 1.4rem;
  color: inherit;
  line-height: 1.6;
  outline: none;

  &:focus,&:active {
    box-shadow: 0 0 0 2px #888;
  }
}
</style>
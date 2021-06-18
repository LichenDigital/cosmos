<template lang="pug">
label(for='standard-dropdown') Dropdown
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
select {
  position: relative;
  appearance: none;
  background: white;
  border: 2px solid black;
  padding: 1rem;
  width: 100%;
  font-family: inherit;
  font-size: inherit;
  cursor: pointer;
  line-height: inherit;
  outline: none;

  &::before {
    position: absolute;
    top: 0;
    left: 0;
    content: '';
    width: 1rem;
    height: 1rem;
    background-color: black;
    // clip-path: polygon(100% 0%, 0% 0%, 50% 100%);
  }

  &:focus,&:active {
    box-shadow: 0 0 0 2px #888;
  }
}
</style>
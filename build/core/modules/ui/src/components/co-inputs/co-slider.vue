<template lang="pug">
//- TODO: Provide a prop for the id for the tickmarks, so that an external dataset can be created, otherwise if left off, create an id dynamically so that it can be used within this component. optionally return that id so that it can be used with another slider...?
datalist(id='tickmarks')
  option(v-for='option, key in dataList' :value='option.value' :label='option.label')
div.co-slider
  coProgress(:bars='[[value]]' :range='max')
  div.co-thumb(:style='{ left: value + "px" }')
  
input(type='range' v-model='value' :max='max' list='tickmarks')
</template>

<script lang="ts">

import { defineComponent } from 'vue';

import coProgress from '../co-progress.vue';

export default defineComponent({
  name: 'coSlider',
  components: {
    coProgress
  },
  data () {
    return {
      value: 20,
      min: 0,
      max: 200,
      step: 20,
      dataList: [
        {
          value: 0,
          label: '0%',
        },
        {
          value: 100,
          label: '50%'
        },
        {
          value: 200,
          label: '100%'
        }
      ]
    };
  }
});

</script>

<style lang="scss">

.co-slider {
  position: relative;
}

.co-thumb {
  position: absolute;
  width: 2rem;
  height: 2rem;
  top: 0;
  background: #555;
}
</style>
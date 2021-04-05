<template lang="pug">
//- TODO: Provide a prop for the id for the tickmarks, so that an external dataset can be created, otherwise if left off, create an id dynamically so that it can be used within this component. optionally return that id so that it can be used with another slider...?
datalist(id='tickmarks')
  option(v-for='option, key in dataList' :value='option.value' :label='option.label')
div.co-slider
  coProgress(@mouseDown='clickHandler($event)' :bars='[bars[0], bars[1]]' :range='max' @select-element='selectHandler')
  div.co-handle(v-if='handles' :style='{ left: value + "px" }')
  
input(v-for='(bar, l) in bars' :key='l' type='range' v-model='bar[1]' :min='bar[0]' :max='max' list='tickmarks')
</template>

<script lang="ts">

import { log } from '../../cosmos';

import { defineComponent } from 'vue';
import coProgress from '../co-progress.vue';

export default defineComponent({
  name: 'coSlider',
  components: {
    coProgress
  },
  data () {
    return {
      bars: [
        [0, 45],
        [120, 160]
      ],
      max: 200,
      step: 20,
      movable: 'end', // movable - (start, end, or both)
        // default - end set normally, can be set to start (this is used both in movable if not specified and is used for equal distance resolution if a click is registered at the exact middle of a bar, aka which side do we move if there are handles on both sides)
      handles: false, // handles - (true, false) - default true
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
  },
  methods: {
    clickHandler(event: any) {
      log('verbose', event);
      const mousePosition: number = event.offsetX;
      const target: any = event.target;
      const progress: any = event.currentTarget;
      const progressWidth: number = progress.offsetWidth;
      const progressCurrentPositionX: number = progress.getBoundingClientRect().x;
      const calculatedValue: number = mousePosition / progressWidth * this.max;
      log('verbose', `mousePosition: ${mousePosition}`);
      log('verbose', `progressWidth: ${progressWidth}`);
      log('verbose', `percentage: ${calculatedValue}`);
      log('verbose', 'target:'); log('verbose', target);
      log('verbose', 'progress:'); log('verbose', progress);
      log('verbose', `progresCurrentPositionX: ${progressCurrentPositionX}`);
      // this.value = calculatedValue;
      // if click on bar, we're working with this bar specifically
        // get the values for the specific bar
        // check if start, end or both are enabled
        switch (this.movable) {
          case 'start':
            // if start then move the start position to match click / drag
            log('normal', 'start is movable');
            break;
          case 'end':
            // if end then move the end position to match the click / drag
            log('normal', 'end is moveable');
            break;
          case 'both':
            // if both then check for which is closest to click, and move that
              // if equal distance (rare but possible) then move default
            log('normal', 'both start and end are moveable');
            break;
        }
      // if click on progress (container/background), we're working with all the bars
        // check for what bar side that is movable is closest to the clicked position
          // so if end is movable, loop through the end positions, and see which one is currently closest
          // and move that end to the point of click by settings its value to that amount
    },
    selectHandler(select: any) {
      log('verbose', 'selectHandler triggered');
      log('verbose', select);
      const mousePosition: number = select.event.offsetX;
      log('verbose', `mousePosition: ${mousePosition}`);
    }
  }
});

</script>

<style lang="scss">

.co-slider {
  position: relative;
}

.co-handle {
  position: absolute;
  width: 2rem;
  height: 2rem;
  top: 0;
  background: #555;
}
</style>
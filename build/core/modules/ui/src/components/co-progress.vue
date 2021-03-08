<template lang="pug">
//- Need to work in optional progress amount indicators inside and externally for the progress bar
.progress
  .bar(v-for='(bar, l) in barsGenerated' :key='l' :style='{left: bar[0], width: bar[1]}')
    //- span {{ bar[0] }} {{ bar[1] }} used currently for debugging, will implement good solution for it soon
</template>

<script lang="ts">

import { defineComponent } from 'vue';

export default defineComponent({
  name: 'coProgress',
  data() {
    return {
      // direction: -1, // Eventually implement default direction
      range: 200,
      bars: [
        // Proceed to switch into one of the two protocols for describing bars:
        // [end] position given by only the end. We start at the 0 position of the range. Examples e.g. [50] in a progress bar that had a range of 200 this would start at the 0 position and go 25% from there. [56%] would start at 0 and go 56% of the range
        // [start, end] positions given by a start and length of range e.g. [34, 86] in a progress bar that had a range of 200 this would start at 17% of the range, and go to 43% of the range.
        [0, 33], // Inidcates bar starts at 0 units and ends at 33 units
        // [65, 50], // Indicates bar starts at 65 units and ends at 50 units / not valid TODO: gotta check for this condition, where the start placement is larger than the end
        // ['50%'], // Indicates start at 0 position of the range, and bar is 50% of the width of the range
        [80], // Indicates start at 0 position of the range, and bar is 80 units wide
        [150, '20%'] // Indicates start at position 150, relative to the range, and the bar ends at 20%
        // [-19, 123], // Numbers that exceed the range are set to the max or min and indicate that overflow has occured somehow
      ]
    }
  },
  computed: {
    barsGenerated(): [string[]] {
      // Figure out if this should be used directly in the template or if it should store the results and then use those stored results in the template
      // TODO: Need to figure out if we should normalize the bars, like this does https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/buffered so that they are ordered, don't overlap, aren't empty, and don't touch (adjacent ranges are folded into a bigger range)?
      // Create bars array to store the bars we're generating
      const barsConverted: [string[]] = [[]];
      const range: number = this.range;
      // Loop through the bars, and generate offsets and percentages
      this.bars.forEach( function(bar, l) {
        // Assign empty array to bar l
        barsConverted[l] = [];
        // Check number of values in each bar. Min 1, Max 2
        if ( bar.length === 0 || bar.length > 2 ) {
          return 'error'; // Need to construct an error / warning handler
        }
        // If bar only has one value, than add the starting position defaulting to 0
        if ( bar.length === 1 ) {
          bar.unshift(0);
        }
        // For each bar, loop through its children and convert values to percentages
        bar.forEach( function(value, i) {
          // Create a variable to store the converted / validated percentage
          let percentageOutput: string;
          // Check if number or string
          console.log(`Checking value ${i} for bar ${l}`);
          switch (typeof value) {
            // If string, gotta check if it's a valid percentage
            case 'string':
              // If not valid percentage: it's 0% or greater and equal or less than 100% then return error
              if ( !value.match(/^((100)|(\d{1,2}(\.\d*)?))%$/gm) ) {
                console.log(`Value provided is not valid percentage, between 0% and 100% inclusive: ${value}`)
                return 'error';
              }
              console.log(`Value is percentage between 0% and 100% inclusive: ${value}`);
              // If value is within 0% and 100% then lets store it in the percentageOutput
              percentageOutput = value;
              break;
            // If number check if 0 or greater and less than or equal to range
            case 'number':
              console.log(`Value is number: ${value}`);
              // Check if number is less than 0 and greater than the range
              if ( value < 0 || value > range) {
                console.log(`Error encountered: value is less than 0 or greater than the range ${value}`);
                // Return error
                return 'error';
              }
              // Convert number to percentage of range
              percentageOutput = ((value / range) * 100) + '%';
              console.log(`Converted number value (${value}) to percentage (${percentageOutput}) of range (${range})`)
              break;
          }
          // Store the converted / validated percentage in the bars array to be returned
          console.log(`Storing value ${i} in bar ${l}`);
          barsConverted[l][i] = percentageOutput;
          console.log(`Stored the percentageOutput (${percentageOutput}) in barsConverted (${barsConverted})`);
        })
      });
      return barsConverted;
    }
  }
})
</script>

<style lang="scss" scoped>

.progress {
  position: relative;
  height: 2rem;
  background: #ccc;
  width: 20rem;
}
.bar {
  position: absolute;
  left: 0;
  top: 0;
  width: 0;
  background: rgba(0,0,0, .4);
  height: 100%;
}

</style>
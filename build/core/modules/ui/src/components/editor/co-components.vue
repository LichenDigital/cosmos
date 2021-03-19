<template lang="pug">
.co-components.co-group
  h2.co-panel-label components
  h3 button
  component(v-for='component, l in components' :v-key='l', :is='component.type', :link='component.link') {{ component.contents[0] }}
  h3 textinput
  coTextInput(:lines='1')
  coTextInput(:lines='2')
  h3 checkbox
  coCheckbox(:checkmark='"image"')
  coCheckbox(:checkmark='"check"')
  h3 radiobutton
  coRadioButton(:group='"group1"', :value='1', :label='1')
  coRadioButton(:group='"group1"', :value='2', :label='2')
  h3 slider
  coSlider
  h3 progress
  //- Progress examples
  //- coProgress(:bars='[[0, "10"], [40, "40%"], [250, 450], ["50%", "60%"]]' :range='90')
  //- coProgress(:bars='[[0, "40%"]]' :range='100')
  div window width: {{ width }} x position: {{ xposition }}
  coProgress(:bars='[[xposition]]' :range='width')
  div window height: {{ height }} y position: {{ yposition }}
  coProgress(:bars='[[yposition]]' :range='height')
  h3 loading
  coLoading
  h3 upload
  h3 link
  h3 datepicker
  h3 calendar
  h3 colorpicker
  h3 graph
  h3 heading
  h3 paragraph
  h3 link
  h3 quote
  h3 list
  h3 table
  h3 icon
  h3 image
  h3 video
  h3 audio
  h3 thumbnail
  h3 dropdown
  h3 navigation
  h3 map


</template>

<script lang="ts">

import { defineComponent } from 'vue';

import coButton from '../co-inputs/co-button.vue';
import coProgress from '../co-progress.vue';
import coLoading from '../co-loading.vue';
import coTextInput from '../co-inputs/co-textinput.vue';
import coCheckbox from '../co-inputs/co-checkbox.vue';
import coRadioButton from '../co-inputs/co-radiobutton.vue';
import coSlider from '../co-inputs/co-slider.vue';

export default defineComponent({
  name: 'components',
  components: {
    coButton,
    coProgress,
    coLoading,
    coTextInput,
    coCheckbox,
    coRadioButton,
    coSlider
  },
  data () {
    return {
      components: [
        { type: 'coButton',
          link: {
            destination: 'https://lichen.co',
            newWindow: true
          },
          contents: [
            'hello'
          ]
        },
        { type: 'coButton',
          link: {
            destination: 'https://vuejs.org',
            newWindow: true
          },
          contents: [
            'world'
          ]
        },
        { type: 'coButton',
          contents: [
            'buttons'
          ]
        }
      ],
      width: 0,
      xposition: 0,
      height: 0,
      yposition: 0,
    }
  },
  methods: {
    mouseMoved(event: any) {
      // On mouse move, get mouse x and y position, and set those to x and y positions in the 
      this.yposition = event.clientY;
      // console.log(`yposition: ${this.yposition}`);
      this.xposition = event.clientX;
      // console.log(`xposition: ${this.xposition}`);
    }
  },
  mounted() {
    // Get size of screen and store
    this.width = window.innerWidth;
    this.height = window.innerHeight;
    // Set mouse move event
    document.addEventListener('mousemove', this.mouseMoved);
  }
})
</script>

<style lang="scss" scoped>
.co-components {
  width: 35%;
  background-color: #e1e1e1;
}
</style>

<template lang="pug">
.co-components.co-group
  h2.co-panel-label components

  h3 button
  component(v-for='component, l in components' :v-key='l', :is='component.type', :link='component.link') {{ component.contents[0] }}
  coButton(@click='addEntry' co-draggable) Test

  video(src='http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/VolkswagenGTIReview.mp4' style='width: 100%; margin-top: 1rem;' co-draggable autoplay loop muted)

  h3 dropdown
  coDropdown


  h3 textinput
  coTextInput(:lines='1')
  coTextInput(:lines='4')


  h3 checkbox
  coCheckbox(:checkmark='"image"', :id='"check1"')
  coCheckbox(:checkmark='"check"', :id='"check2"')


  h3 radiobutton
  coRadioButton(:group='"group1"', :value='1', :label='1')
  coRadioButton(:group='"group1"', :value='2', :label='2')


  h3 slider
  coSlider


  h3 progress
  //- Progress examples
  p single bar
  coProgress(:bars='[[0, "40%"]]' :range='200' :progressBackground='"#966"')
  p multiple bars in one progress
  //- pre coProgress(:bars='[[0, "10"], [40, "60%"], [250, 450], ["50%", "60%"]]' :range='90')
  coProgress(:bars='[[0, "10"], [40, "60%"], [250, 450], ["50%", "60%"]]' :range='90')
  p progress hooked up a single bar to x position of mouse as a demo
  coProgress(:bars='[[xposition]]' :range='width')
  p progress hooked up to two bars both x position of mouse and y position of mouse in the same progress
  //- div window width: {{ width }} x position: {{ xposition }}
  //- div window height: {{ height }} y position: {{ yposition }}
  coProgress(:bars='[[0, xposition], [width, width + yposition]]' :range='height + width')


  h3 loading
  coLoading


  h3 files
  coFiles

  h3 datepicker
  coDatePicker


  h3 calendar


  h3 colorpicker
  coColorPicker


  h3 graph


  h3 heading
  coHeading(:content='"heading 1"' :type='"h1"')
  coHeading(:content='"heading 2"' :type='"h2"')
  coHeading(:content='"heading 3"' :type='"h3"')
  coHeading(:content='"heading 4"' :type='"h4"')
  coHeading(:content='"heading 5"' :type='"h5"')
  coHeading(:content='"heading 6"' :type='"h6"')


  h3 paragraph
  coParagraph Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.


  h3 link


  h3 quote


  h3 list


  h3 table


  h3 icon
  coIcon(:icon='"file"')

  h3 image


  h3 media (audio)
  coMedia


  h3 media (video)


  h3 thumbnail


  h3 navigation


  h3 map




</template>

<script lang="ts">
import { defineComponent } from 'vue';

import coButton from '../controls/co-button.vue';
import coProgress from '../co-progress.vue';
import coLoading from '../co-loading.vue';
import coTextInput from '../controls/co-textinput.vue';
import coCheckbox from '../controls/co-checkbox.vue';
import coRadioButton from '../controls/co-radiobutton.vue';
import coSlider from '../controls/co-slider.vue';
import coParagraph from '../co-paragraph.vue';
import coFiles from '../controls/co-files.vue';
import coIcon from '../co-icon.vue';
import coDatePicker from '../controls/co-datepicker.vue';
import coDropdown from '../controls/co-dropdown.vue';
import coColorPicker from '../controls/co-colorpicker/co-colorpicker.vue';
import coHeading from '../co-heading.vue';
import coMedia from '../co-media.vue';

export default defineComponent({
  name: 'components',
  components: {
    coButton,
    coProgress,
    coLoading,
    coTextInput,
    coCheckbox,
    coRadioButton,
    coSlider,
    coParagraph,
    coFiles,
    coIcon,
    coDatePicker,
    coDropdown,
    coColorPicker,
    coHeading,
    coMedia
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
        },
        { type: 'coButton',
          contents: [
            'hello'
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
    },
    addEntry() {
      this.$store.commit('addEntry', 'test');
    }
  },
  mounted() {
    // Get size of screen and store
    this.width = window.innerWidth;
    this.height = window.innerHeight;
    // Set mouse move event
    document.addEventListener('mousemove', this.mouseMoved);

    // Drag and drop testing
    let dragging = false;
    let draggableItems = document.querySelectorAll('*[co-draggable]');
    if (draggableItems) {
      for (let l = 0; l < draggableItems.length; l++) {
        console.log(draggableItems[l]);
        // Listen for mouse down
        draggableItems[l].addEventListener('mousedown', function(event){
          console.log('mousedown');
          console.log(event);
        });
        // Listen for mouse move
        draggableItems[l].addEventListener('mousemove', function(event) {
          console.log('mousemove');
          console.log(event);
        });
        draggableItems[l].addEventListener('mouseup', function(event) {
          console.log('mouseup');
          console.log(event);
        });
        // Listen for mouse up
      }
    }
  }
})

</script>

<style lang="scss" scoped>
.co-components {
  width: 35%;
  background-color: #e1e1e1;
  @media (max-width: 31.25em) {
    width: 100%
  }
}
</style>

<script>
import { invoke, convertFileSrc } from "@tauri-apps/api/tauri";
import { open } from '@tauri-apps/api/dialog';
import { WebviewWindow } from "@tauri-apps/api/window";
import OKLabColorPicker from "./OKLabColorPicker.vue";
export default
{
  data() {
    return {
      inputPath: "",
      assetPath: "",
      colorPallet: [],
      output: []
     }
  },
  components:{OKLabColorPicker},
  methods: {
    
    updateColor(event, color)
    {
      const index = this.colorPallet.indexOf(color);
      this.colorPallet[index] = event;
      this.active = index;
      console.log(this.active)
    },

    loadSVG : async function()
    {
      this.inputPath = await open({
        directory: false,
        multiple: false,
        filters: [{name: 'Images', extensions: ['svg']}]
      });

      
      console.log(this);
      invoke("get_svg_data", {inputSvgFile: this.inputPath})
      .then((res) => 
      {
        this.colorPallet = res;
      });

      this.assetPath = await convertFileSrc(this.inputPath);
      console.log(this.assetPath);
    },

    newWindow(lable, path)
    {
      const window = new WebviewWindow(lable, {url: path});
      console.log(window.url);
      window.once('tauri://error', function (e) {
        console.log(e);
      })
    }
  }
}
</script>

<template>
  <div class="container">
    <div class="files">
      <button @click="loadSVG()">Load SVG</button>
      <img :src="assetPath" id="main">
    </div>
    <OKLabColorPicker width="100" height="100"/>
    <button @click="colorPallet.push('#000000')">Add color</button>
    
    <button @click="newWindow('Permutate', '/pemutate')">Permutate</button>
    <button @click="">Replace</button> 
    <button @click="">Set Hue</button> 
    

    
    <!-- this does not work. --> 
    <!--
      <div v-for="i in output">
        <img :src="i">
      </div>
    -->
  </div>
</template>

<style scoped>
* 
{
  margin: 10px;
}
input[type="color"]
{
  width: 10vw;
}

.files
{
  display: flex;
  justify-content: space-around;
}

.container
{
  display: flex;
  justify-content: center;
  flex-direction: column
}

.row
{
  display: flex;
  flex-direction: row;
  justify-content: space-between;
}

.colors
{
  display: flex;
  flex-wrap: wrap;
}

#main
{
  width: 30vw;
}
</style>
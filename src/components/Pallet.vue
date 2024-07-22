<script>
import { invoke, convertFileSrc } from "@tauri-apps/api/tauri";
import { open } from '@tauri-apps/api/dialog';
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
  methods: {
    pallet() {
      console.log(this.inputPath);
      invoke("generate_svg_with_color_combinations", { inputSvgFile: this.inputPath, colorPallete: this.colorPallet})
      .then((res) => 
      {
        console.log(res);
        for (let out in res)
        {
          console.log(res[out]);
          const asset = convertFileSrc(res[out]);
          console.log(asset);
          this.output.push(asset); 
        }
      });
    },
    replace() {
      console.log(this.inputPath);
      invoke("replace_svg_color", { inputSvgFile: this.inputPath, colorPallete: this.colorPallet})
      .then((res) => 
      {
        console.log(res);
      })
    },
    shift(type)
    {
      console.log(this.inputPath);
      invoke("oklab_shift", { inputSvgFile: this.inputPath, shiftType: type, shiftColor: this.colorPallet[0]})
      .then((res) => 
      {
        console.log(res);
      })
    },
    updateColor(event, color)
    {
      const index = this.colorPallet.indexOf(color);
      this.colorPallet[index] = event;
      console.log(this.inputPath)
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
    
    <button @click="colorPallet.push('#000000')">Add color</button>
    <div class="colors">
      <div v-for="color in colorPallet" :key="color">
        <color-picker format="hex" @update:pureColor="updateColor($event, color)" :pureColor="color"/>
      </div>
    </div>
    <button @click="pallet()">Permutate</button>
    <button @click="replace()">Replace</button> 
    <div class="row">
      <button @click="shift(0)">Set Chroma (uses the first color)</button> 
      <button @click="shift(1)">Set Hue (uses the first color)</button> 
      <button @click="shift(2)">Set Luminance (uses the first color)</button> 
    </div>

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
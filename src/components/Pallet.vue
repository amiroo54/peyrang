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
    updateColor(event, color)
    {
      const index = this.colorPallet.indexOf(color);
      this.colorPallet[index] = event.target.value;
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
        <input type="color" :value="color" @input="updateColor($event, color)">
      </div>
    </div>
    <button @click="pallet()">Do it</button> 
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
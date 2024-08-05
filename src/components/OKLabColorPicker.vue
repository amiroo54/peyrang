<template>
    <div class="color-picker">
      <canvas
        ref="canvas"
        @mousedown="startPicking"
        @mousemove="picking"
        @mouseup="stopPicking"
        @mouseleave="stopPicking"
      ></canvas>
      <input type="range" v-model="luminance" min="0" max="1" step="0.01" @input="updateColor" />
      <div class="color-display" :style="{ backgroundColor: color }"></div>
    </div>
</template>
  
<script>
export default {
data() {
    return {
    hue: 0,
    chroma: 0,
    luminance: 0.5, // Initial luminance
    color: 'rgb(255, 255, 255)',
    picking: false,
    };
},
mounted() {
    this.drawColorWheel();
},
methods: {
    drawColorWheel() {
    const canvas = this.$refs.canvas;
    const ctx = canvas.getContext('2d');
    const radius = canvas.width / 2;

    for (let angle = 0; angle < 360; angle++) {
        const rad = (angle * Math.PI) / 180;
        ctx.beginPath();
        ctx.moveTo(radius, radius);
        ctx.arc(radius, radius, radius, rad, rad + Math.PI / 180);
        ctx.fillStyle = hsl(angle, 100, 50);
        ctx.fill();
    }
    },
    startPicking(event) {
    this.picking = true;
    this.pickColor(event);
    },
    stopPicking() {
    this.picking = false;
    },
    picking(event) {
    if (this.picking) {
        this.pickColor(event);
    }
    },
    pickColor(event) {
    const canvas = this.$refs.canvas;
    const rect = canvas.getBoundingClientRect();
    const x = event.clientX - rect.left;
    const y = event.clientY - rect.top;
    const radius = canvas.width / 2;

    const dx = x - radius;
    const dy = y - radius;
    const distance = Math.sqrt(dx * dx + dy * dy);
    const angle = Math.atan2(dy, dx) * (180 / Math.PI);

    if (distance <= radius) {
        this.hue = (angle + 360) % 360;
        this.chroma = distance / radius;
        this.updateColor();
    }
    },
    updateColor() {
    const RGB = this.hslToRgb(this.hue, this.chroma, this.luminance);
    this.color = rgb(RGB.r, RGB.g, RGB.b);
    },
    hslToRgb(h, s, l) {
    let r, g, b;

    const chroma = (1 - Math.abs(2 * l - 1)) * s;
    const x = chroma * (1 - Math.abs(((h / 60) % 2) - 1));
    const m = l - chroma / 2;

    if (h < 60) {
        r = chroma; g = x; b = 0;
    } else if (h < 120) {
        r = x; g = chroma; b = 0;
    } else if (h < 180) {
        r = 0; g = chroma; b = x;
    } else if (h < 240) {
        r = 0; g = x; b = chroma;
    } else if (h < 300) {
        r = x; g = 0; b = chroma;
    } else {
        r = chroma; g = 0; b = x;
    }

    return {
        r: Math.round((r + m) * 255),
        g: Math.round((g + m) * 255),
        b: Math.round((b + m) * 255)
    };
    },
},
};
</script>

<style>
.color-picker {
display: flex;
flex-direction: column;
align-items: center;
}

canvas {
background-color: white;
border: 1px solid #ccc;
border-radius: 50%;
width: 300px;
height: 300px;
}

input[type="range"] {
margin: 10px 0;
}

.color-display {
width: 100px;
height: 100px;
border: 1px solid #ccc;
border-radius: 4px;
}
</style>

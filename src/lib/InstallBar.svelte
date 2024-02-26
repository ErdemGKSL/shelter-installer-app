<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  const svgPathMap = {
    "std": "/discord.svg",
    "ptb": "/ptb.svg",
    "canary": "/canary.svg",
  } as const;

  const titleMap = {
    "std": "Normal",
    "ptb": "PTB",
    "canary": "Canary"
  } as const;

  const colorMap = {
    "std": "#5662f6",
    "ptb": "#3662c6",
    "canary": "#faac2a"
  } as const;

  export let type: "std" | "ptb" | "canary" = "std";

  let installed = false;
  let exists = false;

  async function isInstalled() {
    let [e, i] =  await invoke("is_installed", {
      discordType: type.at(0)
    }) as any;

    exists = e;
    installed = i;
  }

  isInstalled();

</script>

<div class="bar {exists ? "enabled" : "disabled"}">
  <div class="signature">
    <span style="background-color: {installed ? "#00ff00" : "#ff0000"};"/>
    <img src={svgPathMap[type]} alt={type}>
    <h1 style="color: {colorMap[type]};">{titleMap[type]}</h1>
  </div>
  <div class="buttons"></div>
</div>

<style lang="scss">

  .disabled {
    opacity: 50%;
    pointer-events: none;
  }

  .bar, .enabled {
    width: 90%;
    height: 100px;
    overflow: hidden;
    border-radius: 10px;
    background-color: #0a4c44;
    // box-shadow: 3px 0px 6px 0px black;
    border: solid #4a8c84;
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    padding: 5px;
    transition: 250ms all;

    &:hover {
      background-color: #003f37;
    }

    .signature {
      display: flex;
      align-items: center;
      flex-direction: row;
      padding: 0px 10px;
      height: 100%;
      width: 300px;
      gap: 10px;
      transition: 50ms;
      // filter: drop-shadow(0 0 0.5em black);

      span {
        background-color: red;
        width: 20px;
        height: 20px;
        border-radius: 100%;
        // margin-right: 10px;
      }

      img {
        user-select: none;
        height: 60%;
      }

      h1 {
        user-select: none;
      }
    }

  }
</style>
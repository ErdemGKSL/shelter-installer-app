<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { message } from "@tauri-apps/api/dialog";

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
  let pending = false;

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
    {#if exists}
      <span style="background-color: {installed ? "#00ff00" : "#ff0000"};"/>
    {/if}
    <img src={svgPathMap[type]} alt={type}>
    <h1 style="color: {colorMap[type]};">{titleMap[type]}</h1>
  </div>
  {#if exists}
    <div class="buttons">
      <button class="{installed ? "delete" : "install"}{pending ? " disabled": ""}" on:click={async () => {
        pending = true
        let a =  await invoke("toggle_inject", {
          discordType: type.at(0)
        }).catch((e) => {
          message(e + " (Try closing discord)", { title: 'Injection', type: 'error' });
        })
        await isInstalled();
        pending = false;
        console.log(a);
      }}>
        {installed ? "Delete" : "Install"}
      </button>
      <button class="update{pending ? " disabled": ""}" on:click={async () => {
        pending = true
        let a =  await invoke("update", {
          discordType: type.at(0)
        }).catch((e) => {
          message(e + " (Try closing discord)", { title: 'Injection', type: 'error' });
        })
        await isInstalled();
        pending = false;
        console.log(a);
      }}>
        Update
      </button>
    </div>
  {:else}
    <h2>
      Not Installed or Not Found
    </h2>
  {/if}
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

    .buttons {
      display: flex;
      flex-direction: row;
      gap: 15px;
      padding: 0px 15px;


      button {
        border-radius: 10px;
        padding: 10px;
        width: 80px;
        border: none;
        pointer-events: all;
        cursor: pointer;
      }

      .delete {
        background-color: #ff0000;
        transition: all 250ms;

        &:hover {
          background-color: #cc0000;
        }
      }

      .install {
        background-color: #00ff00;

        &:hover {
          background-color: #00cc00;
        }
      }

      .update {
        background-color: #ffff00;

        &:hover {
          background-color: #cccc00;
        }
      }

      .disabled {
        pointer-events: none;
        opacity: 50%;
      }
    }

  }
</style>
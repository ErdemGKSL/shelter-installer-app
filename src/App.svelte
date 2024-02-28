<script lang="ts">
  
	import InstallBar from './lib/InstallBar.svelte';
</script>

<main class="container">
  
  <InstallBar type="std"/>
  <InstallBar type="ptb"/>
  <InstallBar type="canary"/>

</main>

<style>

  .container {
    height: 100%;
    width: 100%;
    padding: 30px;
    gap: 30px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: start;
    background-image: linear-gradient(#2a3d4c, #2a7066)
  }

</style>
<script lang="ts">
	import Logo from './Images/LogoGreen.png'

	export default {
		data() {
			return {
				Logo: Logo,
			}
		}
	}
</script>

<template>
	<main data-tauri-drag-region>
		<div class="navigation-container">
			<div class="tray">
				<img :src="Logo" id="logo"/>

				<router-link class="navigation-button" to="/" active-class="active">
					<div class="icon">􀊫</div>
					<div class="text" style="flex-grow: 1;">Search Fonts</div>
				</router-link>

				<router-link class="navigation-button" to="/system-fonts" active-class="active">
					<div class="icon">􀅒</div>
					<div class="text" style="flex-grow: 1;">System Fonts</div>
				</router-link>

				<router-link class="navigation-button" to="/font-boxes" active-class="active">
					<div class="icon">􀐚</div>
					<div class="text" style="flex-grow: 1;">Font Boxes</div>
				</router-link>

				<router-link class="navigation-button" to="/settings" active-class="active">
					<div class="icon">􀣋</div>
					<div class="text" style="flex-grow: 1;">Settings</div>
				</router-link>
			</div>
		</div>

		<div class="content-container">
			<div class="content">
				<router-view></router-view>
			</div>
		</div>
	</main>
</template>

<style scoped>
  main {
    display: flex;
    height: 100%;
    width: 100%;
  }

  .navigation-container {
    width: 250px;
    padding: 16px;
  }

  .content-container {
    flex-grow: 1;
    padding: 16px 16px 16px 0px;
  }

  .content {
    width: 100%;
    height: 100%;
    padding: 32px;

    background-color: white;
    border-radius: 10px;

	display: flex;
	flex-direction: column;
	justify-content: center;
	align-items: center;
  }

  #logo {
    width: 200px;
    display: block;
    margin: 0 auto;
    margin-top: 16px;
    margin-bottom: 16px;
  }

  .active div {
	background: #1B575D !important;
    color: white;
  }

  .icon {
    margin-right: 8px;
    width: 40px;
    height: 40px;
    text-align: center;
    line-height: 40px;
	font-weight: normal;
  }

  .navigation-button {
    width: 100%;
    display: flex;
    padding: 0px 16px 8px 16px;
    border-radius: 500px;

    font-family: 'Inter Tight', sans-serif;
    font-weight: 700;
    color: black;
  }

  .navigation-button:hover div {
	background: rgba(27, 87, 93, 0.25);
    cursor: pointer;
  }

  .navigation-button div {
    border-radius: 500px;
    transition: 0.15s ease all;
  }

  .navigation-button .text {
    padding-left: 16px;
    line-height: 40px;
  }
</style>

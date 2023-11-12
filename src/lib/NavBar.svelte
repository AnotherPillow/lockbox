<script lang="ts">
    import HashKey from "./HashKey.svelte";
    
    import { invoke } from "@tauri-apps/api/tauri"
    import { decrypted, selected_account, stored_key, defaultDecrypted } from "../stores";

    import { toast } from 'svoast';

    let decryptedData = defaultDecrypted
    decrypted.subscribe(value => {
        decryptedData = value
    })

    let storedKey = ''
    stored_key.subscribe(value => {
        storedKey = value
    })
</script>

<nav>
    <ul>
        <li class="logo-text" data-hide-bg>
            Lockbox
        </li>
        <li data-hide-bg>
            <button id="save-btn" on:click={async () => {
                console.log('trying to save...')
                toast.info('Saving...')
                await invoke("write_file", {
                    key: storedKey,
                    data: JSON.stringify(decryptedData)
                })
                toast.success('Saved')
            }}>
                Save
            </button>
        </li>
        <li data-hide-bg>
            <HashKey />
        </li>
    </ul>
</nav>

<style lang="scss">
    nav {
        height: 100px;

        button {
            padding-top: 0.5em;
            padding-bottom: 0.5em;
        }
        
        ul {
            list-style-type: none;
            
            padding: 0;
            overflow: hidden;
            /*background-color: #282828;*/

            

            display: flex;
            flex-direction: row;
            justify-content: space-between;
            align-items: center;
        }

        li {
            // &:not([data-hide-bg]) {
            //     background-color: #242424;
            // }
            
            flex: 1;
            text-align: center;
            margin: 0 0.5em;
            border-radius: 0.5em;

            &.logo-text {
                background-color: #0f0f0f98;
                
                font-size: 1em;
                font-weight: 500;
                font-family: inherit;
                padding: 0.5em 1.2em;
                
                
                box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
            }
        }

        #save-btn {
            background-color: rgb(10, 130, 10);
        }
    }

    

    

</style>
<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri"
    import { decrypted, selected_account, stored_key, defaultDecrypted, futureIndex } from "../stores";
    import { blankAccount, capitalise } from "../util";
    
    import type { accountType } from "../stores"
    
    import { toast } from 'svoast';
    
    let decryptedData = defaultDecrypted
    decrypted.subscribe(value => {
        decryptedData = value
    })
    
    let selectedAccount = -1
    selected_account.subscribe(value => {
        selectedAccount = value
    })

    const accountKeys = () => Object.keys(decryptedData.accounts[selectedAccount])

    async function addnew() {
        if (futureIndex < 0) return;

        let newAccount = structuredClone(blankAccount)
        
        decrypted.set({
            accounts: [
                ...decryptedData.accounts, newAccount as accountType
            ]
        })
    }

    async function chooseAccount(index: number) {
        selected_account.set(index)
    }
</script>

<!--make a grid with 1 small column and 2 big ones-->
<div class="gridparent">
    <div class="column gridlistarea">
        <button class="btn btn-primary" on:click={addnew}>Add</button>
        <!--make an UL thats children is bound to the listElems array-->
        <ul id="accounts">
            {#each decryptedData.accounts as acc, index}
                {#if acc !== undefined && JSON.stringify(acc) !== '{}' && acc.title !== '@~lb~hidden'}
                    <li data-account-index={index}>
                        <button on:click={() => chooseAccount(index)} class="invis-button">
                            <h4>
                                {acc.title}
                            </h4>
                        </button>
                    </li>
                {/if}
            {/each}
        </ul>
    </div>
    <div class="column maingridarea">
        {#if selectedAccount == -2}
            <p>Please enter your master password and then select an account to see details here.</p>
        {:else if selectedAccount == -1}
            <p>Please select an account to see details here.</p>
        {:else}
            <input 
                class="invis-input account-title invis-input-transparent"
                bind:value={decryptedData.accounts[selectedAccount].title}
                spellcheck="false"
            />
            {#each accountKeys() as key}
                <span style="display: none;">{key}</span>
                <div class="account-details-container">
                    <label 
                        for={key + '-input'}
                        class="account-detail-label"
                    >{capitalise(key)}: </label>
                    {#if key == 'notes'}
                        <textarea 
                            data-account-index={selectedAccount} 
                            data-account-field={key} 
                            name={key + '-input'} 
                            id={key + '-input'}
                            class="account-detail-input"
                            placeholder={`Your ${capitalise(key)}...`}
                            bind:value={decryptedData.accounts[selectedAccount][key]}
                        />
                    {:else} 
                        <input 
                            data-account-index={selectedAccount} 
                            data-account-field={key} 
                            name={key + '-input'} 
                            id={key + '-input'}
                            type="text"
                            class="account-detail-input"
                            placeholder={`Your ${capitalise(key)}...`}
                            bind:value={decryptedData.accounts[selectedAccount][key]}
                        />
                    {/if}
                </div>
            {/each}
            <button class="account-delete-btn" on:click={() => {
                decryptedData.accounts[selectedAccount].title = '@~lb~hidden'
                setTimeout(() => {
                    window.location.reload()
                }, 250)
            }}>Delete</button>
        {/if}
    </div>
</div>

<style lang="scss">

    .account-title {
        font-size: 2em;
        width: 100%;
        text-align: center;
    }

    .account-delete-btn {
        background-color: darkmagenta;
        
        position: absolute;
        bottom: 15px;
    }

    .gridparent {
        display: grid;
        grid-template-columns: 1fr 2fr;
        grid-template-rows: 1fr;
        grid-template-areas: "gridlistarea maingridarea";

        gap: 0.5em;

        height: calc(100vh - 116px);
        width: 100%;

        .gridlistarea {
            grid-area: gridlistarea;

            .btn {
                width: 100%;
            }

            ul {
                list-style-type: none;

                text-align: center;
                padding:0;

                &:not(:has(h1)) {
                    font-size: x-large
                }

                &:has(li) * { /* needed to prevent warning */
                    margin: 0.5em 0;
                }

                li {
                    cursor: pointer;
                }
            }
        }

        .maingridarea {
            grid-area: maingridarea;
            position: relative;

            .account-details-container {
                display: flex;
                justify-content: space-between;
                width: 100%;
                margin-bottom: 10px; /* Optional spacing between input pairs */

                .account-detail-label {
                    text-align: left;
                    flex: 1;
                    white-space: nowrap;
                    overflow: hidden;
                    text-overflow: ellipsis;
                    padding-right: 10px; /* Optional spacing between label and input */
                }

                .account-detail-input {
                    text-align: center;
                    flex: 1;
                }       
            }
        }

        .column {
            background-color: #0f0f0f98;
            border-radius: 0.5em;
            box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
            padding: 0.5em 1.2em;
        }
    }
</style>
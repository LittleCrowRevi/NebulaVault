using System.Collections.Generic;
using System.Linq;
using ObjectExtensions;
using TMPro;
using UnityEngine;
using UnityEngine.InputSystem;
using UnityEngine.SceneManagement;
using UnityEngine.Serialization;

//TODO: Maybe make a battle an overlay over the current scene with the background blacked out? Would remove the need to reload a scene and ease data transfer...

public enum BattlingState
{
    Start,
    PlayerTurn,
    SelectingTarget,
    EnemyTurn,
    Win,
    Loss
}

public class BattleState : MonoBehaviour, IState
{
    public BattlingState state;

    public List< GameObject >  Friendly  { get; set; } = new List< GameObject >();
    public List< GameObject >  Hostile   { get; set; } = new List< GameObject >();
    public Stack< GameObject > TurnOrder { get; set; } = new Stack< GameObject >();

    public GameObject battleUi;
    public GameObject enemyUi;

    public Game            Game            { get; set; }
    public string          Name            { get; set; } = "Battle State";
    public StateController StateController { get; set; }

    public void Enter()
    {
        SceneManager.LoadSceneAsync( "Battleground_Dungeon" );

        SceneManager.sceneLoaded                      += OnSceneLoaded;
        StateController.m_AttackCommand.OnEventRaised += OnAttackCommand;

        state = BattlingState.Start;
    }

    private void OnSceneLoaded( Scene scene, LoadSceneMode mode )
    {
        battleUi = Instantiate( StateController.resourceLoader.BattleUiPrefab );

        var battleground = scene.GetRootGameObjects().Single( o => o.CompareTag( "Battleground" ) ).GetComponent< Battleground >();
        for ( int i = 0; i < Friendly.Count; i++ )
        {
            Friendly[ i ].transform.position = battleground.FriendlyAnchors[ i ].transform.position;
        }

        //TODO: make seperate uis for each enemy slot, giving them their dedicated EventSO and such to streamline instantiation?
        for ( int i = 0; i < Hostile.Count; i++ )
        {
            var healthEvent = Instantiate( StateController.resourceLoader.IntEventChannelSO );

            Hostile[ i ] = Instantiate( Hostile[ i ] );
            var hostileEntity = Hostile[ i ].GetComponent< Entity >();
            hostileEntity.m_healthChange = healthEvent;

            var euiPrefab = Resources.Load< GameObject >( "UI/EnemyUI" );
            enemyUi = Instantiate( euiPrefab, battleUi.transform );

            var enemyUiScript = enemyUi.GetComponent< EnemyUI >();
            enemyUiScript.m_healthChange = healthEvent;
            enemyUiScript.observedEntity = hostileEntity;

            Hostile[ i ].transform.position = battleground.HostileAnchors[ i ].transform.position;
        }

        StateController.m_ChangeCameraTarget.RaiseEvent( battleground.gameObject );
        StateController.m_StartingBattle.RaiseEvent();

        state = BattlingState.PlayerTurn;
    }

    public void Exit()
    {
        SceneManager.sceneLoaded -= OnSceneLoaded;
        if ( StateController.m_AttackCommand.IsValid() ) StateController.m_AttackCommand.OnEventRaised -= OnAttackCommand;

        SceneManager.LoadSceneAsync( StateController.gameData.LastActiveOverworldScene );
        StateController.m_ExitingBattle.RaiseEvent();
        StateController.m_ChangeCameraTarget.RaiseEvent( Game.player );
    }

    private void OnAttackCommand()
    {
        Debug.Log( "Attack pressed!" );
        var atk = Friendly[ 0 ].GetComponent< Entity >().entityData.body;

        var hostile = Hostile[ 0 ];
        hostile.GetComponent< Entity >().TakeDamage( atk );

        SpawnFloatingText( hostile, $"{atk}" );

        state = BattlingState.EnemyTurn;
    }

    public void Update()
    {
        if ( state == BattlingState.EnemyTurn )
        {
            var atk = Hostile[ 0 ].GetComponent< Entity >().entityData.body;

            var friendly = Friendly[ 0 ];
            friendly.GetComponent<Entity>().TakeDamage( atk );
            
            SpawnFloatingText( friendly, $"{atk}" );

            state = BattlingState.PlayerTurn;
        }
    }

    private void SpawnFloatingText( GameObject target, string text )
    {
        var dmgText = Instantiate( Game.floatingDamageTextPrefab, target.transform.position, target.transform.rotation, Game.worldCanvas.transform );
        dmgText.GetComponent< TextMeshProUGUI >().SetText( text );
    }
}
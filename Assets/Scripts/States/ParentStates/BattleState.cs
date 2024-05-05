using System.Collections.Generic;
using System.Linq;
using UnityEngine;
using UnityEngine.InputSystem;
using UnityEngine.SceneManagement;

public enum BattlingState
{
    Start,
    PlayerTurn,
    SelectingTarget,
    EnemyTurn,
    Win,
    Loss
}

public class BattleState : IState
{
    /// signals
    /// methods
    public BattleState()
    {
    }

    public BattlingState state;

    public List< GameObject >  Friendly  { get; set; } = new List< GameObject >();
    public List< GameObject >  Hostile   { get; set; } = new List< GameObject >();
    public Stack< GameObject > TurnOrder { get; set; } = new Stack< GameObject >();

    public Game            Game            { get; set; }
    public string          Name            { get; set; } = "Battle State";
    public StateController StateController { get; set; }

    public void Enter()
    {
        SceneManager.LoadSceneAsync( "Battleground_Dungeon" );
        SceneManager.sceneLoaded += OnSceneLoaded;

        state = BattlingState.Start;
    }

    private void OnSceneLoaded( Scene scene, LoadSceneMode mode )
    {
        var battleground = scene.GetRootGameObjects().Single( o => o.CompareTag( "Battleground" ) ).GetComponent< Battleground >();
        for ( int i = 0; i < Friendly.Count; i++ )
        {
            Friendly[ i ].transform.position = battleground.FriendlyAnchors[ i ].transform.position;
        }

        for ( int i = 0; i < Hostile.Count; i++ )
        {
            Hostile[ i ].transform.position = battleground.HostileAnchors[ i ].transform.position;
        }

        StateController.m_ChangeCameraTarget.RaiseEvent( battleground.gameObject );
        StateController.m_LoadBattleUi.RaiseEvent();

        StateController.m_StartingBattle.RaiseEvent();

        state = BattlingState.PlayerTurn;
    }

    public void Exit()
    {
        SceneManager.sceneLoaded -= OnSceneLoaded;
        SceneManager.LoadSceneAsync( StateController.GameData.LastActiveOverworldScene );
        StateController.m_ExitingBattle.RaiseEvent();
        StateController.m_ChangeCameraTarget.RaiseEvent( Game.player );
    }

    public void Update()
    {
    }
}
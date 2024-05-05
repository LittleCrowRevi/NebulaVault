using System.Linq;
using ObjectExtensions;
using UnityEngine;
using UnityEngine.SceneManagement;

public class ExplorationState : IState
{
    public Vector3? CachedPlayerPosition { get; set; }

    public Game            Game            { get; set; }
    public string          Name            { get; set; } = "Exploration State";
    public StateController StateController { get; set; }

    public void Enter()
    {
        Game.player.transform.position = CachedPlayerPosition ??= Game.player.transform.position;

        Game.m_OnInitiateBattle.OnRaiseEvent += OnInitiateBattle;

        SceneManager.sceneLoaded += OnSceneLoaded;
    }

    public void Update()
    {
        CachedPlayerPosition = Game.player.transform.position;
    }

    public void Exit()
    {
        Game.m_OnInitiateBattle.OnRaiseEvent -= OnInitiateBattle;

        SceneManager.sceneLoaded -= OnSceneLoaded;
    }

    private void OnInitiateBattle( GameObject[] friendlyActors, GameObject[] hostileActors )
    {
        Debug.Log( "Starting battle transition." );
        var battleState = new BattleState
        {
            Hostile  = hostileActors.ToList(),
            Friendly = friendlyActors.ToList()
        };

        if ( StateController.GameData )
        {
            StateController.GameData.PlayerPosition           = CachedPlayerPosition ??= Game.player.transform.position;
            StateController.GameData.LastActiveOverworldScene = SceneManager.GetActiveScene().name;
        }

        Game.m_StateChange.RaiseEvent( battleState, TransitionType.Add );
    }

    private void OnSceneLoaded( Scene scene, LoadSceneMode mode )
    {
        StateController.m_LoadUi.RaiseEvent();
    }
}
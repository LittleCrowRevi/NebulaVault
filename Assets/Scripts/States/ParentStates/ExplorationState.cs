using System.Linq;
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
    }

    public void Update()
    {
        CachedPlayerPosition = Game.player.transform.position;
    }

    public void Exit()
    {
        Game.m_OnInitiateBattle.OnRaiseEvent -= OnInitiateBattle;
    }

    private void OnInitiateBattle( GameObject[] friendlyActors, GameObject[] hostileActors )
    {
        Debug.Log( "Starting battle transition." );
        var battleState = new BattleState
        {
            Hostile  = hostileActors.ToList(),
            Friendly = friendlyActors.ToList()
        };

        Game.m_StateChange.RaiseEvent( battleState, TransitionType.Add );
    }
}
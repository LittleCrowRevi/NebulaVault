using UnityEngine;

public class ExplorationState : IState
{
    public Vector3? CachedPlayerPosition { get; set; }

    public string Name { get; set; } = "Exploration State";

    public void Enter()
    {
        //Game.Player.Position = CachedPlayerPosition ??= Game.Player.Position;
    }

    public void Update()
    {
        //CachedPlayerPosition = Game.Player.Position;
    }

    public void Exit()
    {
    }
}
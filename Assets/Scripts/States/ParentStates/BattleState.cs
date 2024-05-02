using System.Collections.Generic;
using UnityEngine;

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
    public BattleState( StateController stateController, Game gameManager, Battleground battleground )
    {
        Game            = gameManager;
        Battleground    = battleground;
        StateController = stateController;
    }

    public BattlingState state;

    public List< GameObject >  Friendly  { get; set; } = new List< GameObject >();
    public List< GameObject >  Hostile   { get; set; } = new List< GameObject >();
    public Stack< GameObject > TurnOrder { get; set; } = new Stack< GameObject >();

    //public Battle Battleground { get; set; }

    public Game            Game            { get; set; }
    public StateController StateController { get; set; }
    public Battleground    Battleground    { get; set; }

    public string Name { get; set; } = "Battle State";

    public void Enter()
    {
        for ( int i = 0; i < Friendly.Count; i++ )
        {
            Friendly[ i ].transform.position = Battleground.FriendlyAnchors[ i ].transform.position;
        }

        for ( int i = 0; i < Hostile.Count; i++ )
        {
            Hostile[ i ].transform.position = Battleground.HostileAnchors[ i ].transform.position;
        }

        Game.m_ChangeCameraTarget.RaiseEvent( Battleground.gameObject );

        state = BattlingState.PlayerTurn;
    }

    public void Update()
    {
        /*if ( Input.IsActionJustPressed( "dev_battle" ) )
        {
            // Make into ExitBattle Event/Signal
            Game.Camera.EmitSignal( GlobalCamera.SignalName.TargetChange, Game.Player );
            Game.GetNode< Node2D >( "BattleScene" ).GetChild( 0 ).Free();

            Game.StateController.EmitSignal( StateController.SignalName.StateChange, ( int )TransitionType.Remove, new ToolbarMenu.Variant() );
            Game.GetNode< Node2D >( "WorldScene" ).Visible = true;
        }*/
    }

    public void Exit()
    {
        
        Game.m_ChangeCameraTarget.RaiseEvent( Game.player );
    }
}
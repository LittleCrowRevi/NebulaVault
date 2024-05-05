using System;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

public enum TransitionType
{
    /// <summary>
    /// Add the new State upon the stack without removing the previous State.
    /// </summary>
    Add,

    /// <summary>
    /// Replace the previous State on the stack with the new State.
    /// </summary>
    Replace,

    /// <summary>
    /// Remove the current State from the stack.
    /// </summary>
    Remove
}

// implement transition methods? from to?
public class StateController : MonoBehaviour
{
    public IState CurrentState => PeekState();
    public Game   Game;

    private readonly Stack< IState >    _stateStack  = new();
    private readonly List< Transition > _transitions = new();

    [Header( "Data" )]
    [SerializeField] public GameDataSO GameData;
    
    [Header( "Broadcast Events" )]
    [SerializeField] public GameObjectEventChannelSO m_ChangeCameraTarget;

    [SerializeField] public VoidEventChannelSO m_StartingBattle;
    [SerializeField] public VoidEventChannelSO m_ExitingBattle;
    [SerializeField] public VoidEventChannelSO m_LoadUi;

    [Header( "Listen to Events" )]
    [SerializeField] public ChangeStateEventChannelSO m_StateChange;

    [SerializeField] public VoidEventChannelSO m_LoadBattleUi;

    private void OnEnable()
    {
        if ( m_StateChange is not null )
        {
            m_StateChange.OnEventRaised += OnTransition;
        }

        DontDestroyOnLoad( gameObject );
    }

    private void Update()
    {
        if ( _transitions?.Count > 0 ) ResolveTransitions( _transitions );
        CurrentState?.Update();
    }

    private IState PeekState()
    {
        return _stateStack.Count > 0 ? _stateStack.Peek() : null;
    }

    private void ResolveTransitions( IReadOnlyList< Transition > transitions )
    {
        if ( transitions.Count == 0 ) return;
        for ( var index = 0; index < transitions.Count; index++ )
        {
            var transition = transitions[ index ];
            if ( transition == null ) continue;
            switch ( transition.TransitionType )
            {
                case TransitionType.Replace:
                    transition.Next.Game            = Game;
                    transition.Next.StateController = this;
                    ReplaceState( transition.Next );
                    _transitions.Remove( transition );
                    break;

                case TransitionType.Add:
                    transition.Next.Game            = Game;
                    transition.Next.StateController = this;
                    AddState( transition.Next );
                    _transitions.Remove( transition );
                    break;

                case TransitionType.Remove:
                    RemoveState();
                    _transitions.Remove( transition );
                    break;

                default:
                    throw new ArgumentOutOfRangeException( nameof( transitions ), "TransitionType missing" );
            }

            Debug.Log( "Transitioned to State: " + transition.Next.Name );
        }
    }

    public void InitialState( IState state )
    {
        _stateStack.Clear();
        _stateStack.Push( state );

        CurrentState?.Enter();
    }

    private void AddState( IState nextState )
    {
        if ( nextState == CurrentState ) return;

        CurrentState?.Exit();

        _stateStack.Push( nextState );
        CurrentState?.Enter();
    }

    private void RemoveState()
    {
        var stackEmpty = _stateStack is not { Count: > 1 };
        if ( stackEmpty ) return;

        CurrentState?.Exit();
        _stateStack.Pop();

        CurrentState?.Enter();
    }

    private void ReplaceState( IState nextState )
    {
        if ( nextState == CurrentState ) return;

        CurrentState?.Exit();
        if ( _stateStack.Count > 0 ) _stateStack.Pop();
        _stateStack.Push( nextState );
        CurrentState?.Enter();
    }

    private void OnTransition( IState state, TransitionType transitionType )
    {
        var t = new Transition( transitionType, state );
        _transitions.Add( t );
    }

    private sealed class Transition
    {
        public readonly IState         Next;
        public readonly TransitionType TransitionType;

        public Transition( TransitionType transitionType, IState next )
        {
            Next           = next;
            TransitionType = transitionType;
        }
    }
}
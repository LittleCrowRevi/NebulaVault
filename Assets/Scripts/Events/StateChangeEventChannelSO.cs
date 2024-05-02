using Unity;
using UnityEngine;
using UnityEngine.Events;

[CreateAssetMenu( menuName = "Events/State Event Channel" )]
public class StateChangeEventChannelSO : ScriptableObject
{
    public UnityAction< IState, TransitionType > OnEventRaised;

    public void RaiseEvent( IState state, TransitionType transitionType )
    {
        OnEventRaised?.Invoke( state, transitionType );
    }
}
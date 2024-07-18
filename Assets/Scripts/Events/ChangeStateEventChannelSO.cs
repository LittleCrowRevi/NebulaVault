using Unity;
using UnityEngine;
using UnityEngine.Events;

[CreateAssetMenu( menuName = "Events/State Event Channel" )]
public class ChangeStateEventChannelSO : ScriptableObject
{
    public delegate void Event( IState state, TransitionType transitionType);

    public event Event OnEventRaised;

    public void RaiseEvent( IState state, TransitionType transitionType )
    {
        OnEventRaised?.Invoke( state, transitionType );
    }
}
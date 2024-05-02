using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Serialization;
using ObjectExtensions;

public class Game : MonoBehaviour
{
    [Header( "Data" )]
    [SerializeField] public GameObject player;

    [Header( "Broadcast Events" )]
    [SerializeField] public GameObjectEventChannelSO m_ChangeCameraTarget;

    [SerializeField] public StateChangeEventChannelSO m_StateChange;

    // Start is called before the first frame update
    private void Start()
    {
        player = GameObject.Find( "Player" );

        m_StateChange.IsValid()?.RaiseEvent( new ExplorationState(), TransitionType.Add );

        m_ChangeCameraTarget.IsValid()?.RaiseEvent( player );
    }
}

namespace ObjectExtensions
{
    public static class Extensions
    {
        public static T IsValid< T >( this T unityObject ) where T : UnityEngine.Object
        {
            return !unityObject ? null : unityObject;
        }
    }
}
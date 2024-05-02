using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.InputSystem;
using UnityEngine.Serialization;

public class InputManager : MonoBehaviour
{
    [Header( "Broadcrast Events" )]
    [SerializeField] private VoidEventChannelSO m_OpenStatScreen;
    
    public void OnOpenStatScreen()
    {
        if ( m_OpenStatScreen )
        {
            m_OpenStatScreen.RaiseEvent();
        }
    }
}
